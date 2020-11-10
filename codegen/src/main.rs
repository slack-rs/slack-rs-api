#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::iter::Peekable;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{App, Arg};
use reqwest::blocking::Client;

mod rust;
use rust::{GenMode, HttpMethod, Method, Module, ModuleBuilder, Parameter, Response};

mod schema;
use schema::{EnumValues, Operation, PathItem, Spec};

mod adapt_gen;
use adapt_gen::create_adapt_skeleton;

mod adapt;
use adapt::correct;

mod vec_or_single;

const ADAPT_OUT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/adapt");
const DEFAULT_OUT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src");
const SLACK_API_SCHEMA: &str = "https://api.slack.com/specs/openapi/v2/slack_web.json";

struct Arguments {
    outdir: PathBuf,
}

fn main() -> Result<()> {
    let arguments = handle_arguments()?;
    let mut spec = fetch_slack_api_spec()?;
    spec.replace_refs()?;
    let mut modules = transform_to_modules(&spec)?;
    create_adapt_skeleton(ADAPT_OUT_DIR, &modules)?;
    correct(&mut modules);
    generate(&arguments.outdir, &modules)?;
    Ok(())
}

fn handle_arguments() -> Result<Arguments> {
    let matches = App::new("slack-rs API Code Generator")
        .arg(
            Arg::with_name("out_dir")
                .short("o")
                .long("outdir")
                .value_name("DIR")
                .help("Sets the output directory for the generated code.")
                .default_value(DEFAULT_OUT_DIR)
                .validator_os(|dir| {
                    let outdir = Path::new(dir);
                    if outdir.exists() && !outdir.is_dir() {
                        return Err("must be a directory".into());
                    }
                    Ok(())
                }),
        )
        .get_matches();

    let outdir = PathBuf::from(
        matches
            .value_of_os("out_dir")
            .context("argument `out_dir missing`")?,
    );
    Ok(Arguments { outdir })
}

fn fetch_slack_api_spec() -> Result<Spec> {
    Client::new()
        .get(SLACK_API_SCHEMA)
        .send()
        .context("Unable to send request to slack api")?
        .error_for_status()
        .context("Slack Server send failure code")?
        .json()
        .context("Unable to deserialize slack server response")
}

fn transform_to_modules(spec: &Spec) -> Result<Vec<Module>> {
    let mut modules: HashMap<&str, ModuleBuilder> = HashMap::new();
    for (full_name, path) in &spec.paths {
        let mut module_names = module_iterator(full_name);
        let top_path = first_path_from_iterator(&mut module_names, full_name)?;
        let mut module = add_module_if_not_exists(&mut modules, top_path);
        add_submodules(&mut module_names, &mut module, path, full_name)?;
    }
    Ok(modules.into_iter().map(|(_, v)| v.build()).collect())
}

fn module_iterator<'a>(full_name: &'a str) -> Peekable<impl Iterator<Item = &'a str>> {
    full_name.trim_start_matches('/').split('.').peekable()
}

fn first_path_from_iterator<'a>(
    i: &mut impl Iterator<Item = &'a str>,
    full_name: &str,
) -> Result<&'a str> {
    i.next()
        .with_context(|| format!("Path does not have a top name: {}", full_name))
}

fn add_module_if_not_exists<'a, 'b>(
    modules: &'b mut HashMap<&'a str, ModuleBuilder<'a>>,
    name: &'a str,
) -> &'b mut ModuleBuilder<'a> {
    modules
        .entry(name)
        .or_insert_with(|| ModuleBuilder::new(name))
}

fn add_submodules<'a>(
    module_names: &mut Peekable<impl Iterator<Item = &'a str>>,
    mut module: &mut ModuleBuilder<'a>,
    path: &PathItem,
    full_name: &'a str,
) -> Result<()> {
    while let Some(name) = module_names.next() {
        if module_names.peek().is_some() {
            module = add_module_if_not_exists(&mut module.submodules, name);
        } else {
            if let Some(op) = &path.get {
                add_method_if_not_exists(module, name, op, HttpMethod::Get, full_name)?;
            }
            if let Some(op) = &path.post {
                add_method_if_not_exists(module, name, op, HttpMethod::Post, full_name)?;
            }
        }
    }
    Ok(())
}

fn add_method_if_not_exists<'a>(
    module: &mut ModuleBuilder<'a>,
    name: &'a str,
    op: &Operation,
    http_method: HttpMethod,
    full_name: &'a str,
) -> Result<()> {
    let method = create_method(module, name, op, http_method, full_name)?;
    if let Some(cur) = module.methods.get(name) {
        bail!(format!(
            "Method with the name {} already exists for module {}. \nOld: {:#?}\nNew: {:#?}",
            name, module.name, cur, method,
        ));
    } else {
        module.methods.insert(name, method);
    }
    Ok(())
}

fn create_method(
    module: &mut ModuleBuilder<'_>,
    name: &str,
    op: &Operation,
    http_method: HttpMethod,
    full_name: &str,
) -> Result<Method> {
    let response: schema::Schema = op
        .responses
        .get("200")
        .with_context(|| {
            format!(
                "Method {} in module {} is missing the 200 response",
                name, module.name
            )
        })?
        .schema
        .clone();
    let response = op.responses.values().fold(response, |mut acc, res| {
        acc.merge(&res.schema);
        acc
    });
    let mut errors: Vec<String> = response
        .properties
        .as_ref()
        .and_then(|m| m.get("error"))
        .and_then(|s| s.enum_values.as_ref())
        .map_or_else(Vec::new, |e| {
            e.iter()
                .filter_map(|v| match v {
                    EnumValues::Bool(_) => None,
                    EnumValues::String(s) => Some(s.clone()),
                })
                .collect()
        });
    errors.sort_unstable();
    let response = Response::try_from(&response).with_context(|| {
        format!(
            "Unable to convert type in method {} in module {}",
            name, module.name
        )
    })?;
    let mut method = Method {
        name: name.into(),
        full_name: full_name.into(),
        description: op.description.clone(),
        documentation_url: op.external_docs.url.clone(),
        parameters: Vec::new(),
        response,
        http_method,
        errors,
    };
    for parameter in &op.parameters {
        let parameter = match parameter.location.as_ref() {
            "header" if parameter.name == "token" => Parameter::try_from(parameter)?,
            "formData" | "query" => Parameter::try_from(parameter)?,
            loc => bail!(format!(
                "Unsupported paramter location {} for {}",
                loc, parameter.name
            )),
        };
        method.parameters.push(parameter);
    }
    method
        .parameters
        .sort_unstable_by(|a, b| a.name.cmp(&b.name));

    Ok(method)
}

fn generate(outdir: &Path, modules: &[Module]) -> Result<()> {
    if !outdir.exists() {
        fs::create_dir_all(outdir)
            .with_context(|| format!("Unable to create directory at: {:?}", outdir))?;
    }

    let moddir = outdir.join("mod_types");
    if !moddir.exists() {
        fs::create_dir_all(&moddir)
            .with_context(|| format!("Unable to create directory at: {:?}", moddir))?;
    }
    Module::generate(&modules, &moddir, GenMode::Types)?;

    let moddir = outdir.join("async_impl").join("mods");
    if !moddir.exists() {
        fs::create_dir_all(&moddir)
            .with_context(|| format!("Unable to create directory at: {:?}", moddir))?;
    }
    Module::generate(&modules, &moddir, GenMode::Async)?;

    let moddir = outdir.join("sync").join("mods");
    if !moddir.exists() {
        fs::create_dir_all(&moddir)
            .with_context(|| format!("Unable to create directory at: {:?}", moddir))?;
    }
    Module::generate(&modules, &moddir, GenMode::Sync)?;
    Ok(())
}
