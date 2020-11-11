use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use inflector::Inflector;

use crate::schema;

pub static AUTOGEN_HEADER: &str = "
//=============================================================================
// 
//                    WARNING: This file is AUTO-GENERATED
// 
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
// 
//=============================================================================

";

#[derive(Clone, Debug)]
pub struct ModuleBuilder<'a> {
    pub name: String,
    pub submodules: HashMap<&'a str, ModuleBuilder<'a>>,
    pub methods: HashMap<&'a str, Method>,
}

impl ModuleBuilder<'_> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            submodules: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn build(self) -> Module {
        let mut submodules: Vec<Module> = self
            .submodules
            .into_iter()
            .map(|(_, v)| v.build())
            .collect();
        submodules.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        let mut methods: Vec<Method> = self.methods.into_iter().map(|(_, v)| v).collect();
        methods.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        Module {
            name: self.name,
            submodules,
            methods,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub submodules: Vec<Module>,
    pub methods: Vec<Method>,
}

impl Module {
    pub fn generate(modules: &[Self], moddir: &Path, gen_mode: GenMode) -> Result<()> {
        Self::generate_modrs(modules, moddir, gen_mode)?;
        for module in modules {
            Self::generate_mod(module, moddir, String::new(), gen_mode)?;
        }
        Ok(())
    }

    fn generate_modrs(modules: &[Self], moddir: &Path, gen_mode: GenMode) -> Result<()> {
        let data = format!("{}\n", Self::build_module_include(modules, gen_mode));

        let mod_file_path = moddir.join("mod.rs");
        let mut mod_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file_path)
            .with_context(|| format!("Unable to create file at: {:?}", mod_file_path))?;

        mod_file
            .write_all(data.as_bytes())
            .with_context(|| format!("Failed to write to file: {:?}", mod_file_path))?;
        Ok(())
    }

    fn generate_mod(
        module: &Self,
        moddir: &Path,
        mut type_path: String,
        gen_mode: GenMode,
    ) -> Result<()> {
        let modname = module.name.to_snake_case();
        if !type_path.is_empty() {
            type_path.push_str("::");
        }
        type_path.push_str(&modname);
        let mod_file_path = if module.submodules.is_empty() {
            let postfix = if gen_mode == GenMode::Types {
                "_types"
            } else {
                ""
            };
            type_path.push_str("_types");
            moddir.join(format!("{}{}.rs", modname, postfix))
        } else {
            let moddir = moddir.join(&modname);
            if !moddir.exists() {
                fs::create_dir_all(&moddir)
                    .with_context(|| format!("Unable to create directory at: {:?}", moddir))?;
            }
            for submodule in &module.submodules {
                Self::generate_mod(submodule, &moddir, type_path.clone(), gen_mode)?;
            }
            moddir.join("mod.rs")
        };
        let data = module.generate_mod_data(gen_mode, type_path)?;
        let mut mod_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file_path)
            .with_context(|| format!("Unable to create file at: {:?}", mod_file_path))?;

        mod_file
            .write_all(data.as_bytes())
            .with_context(|| format!("Failed to write to file: {:?}", mod_file_path))?;

        let out = Command::new("rustfmt")
            .args(&["--edition", "2018"])
            .arg(&mod_file_path)
            .output()
            .context("Unable to run rustfmt")?;
        if !out.status.success() {
            bail!("rustfmt failed for {:?}", mod_file_path);
        }

        Ok(())
    }

    fn generate_mod_data(&self, gen_mode: GenMode, path: String) -> Result<String> {
        let modules = if self.submodules.is_empty() {
            String::new()
        } else {
            format!(
                "{modules}\n\n",
                modules = Self::build_module_include(&self.submodules, gen_mode)
            )
        };
        let (body, imports) = if gen_mode == GenMode::Types {
            (
                self.build_types()?,
                "use std::borrow::Cow;
                use std::convert::From;
                use std::error::Error;
                use std::fmt;"
                    .into(),
            )
        } else {
            (
                self.build_calls(gen_mode)?,
                format!(
                    "use std::borrow::Cow;
                    {import}
                    pub use crate::mod_types::{path}::*;",
                    import = gen_mode.import(),
                    path = path,
                ),
            )
        };
        let data = format!(
            "{header}
            #![allow(unused_imports)]
            #![allow(clippy::match_single_binding)]
            #![allow(clippy::blacklisted_name)]
        
            {modules}{imports}
            
            {body}",
            header = AUTOGEN_HEADER,
            modules = modules,
            imports = imports,
            body = body
        );
        Ok(data)
    }

    fn build_module_include(modules: &[Self], gen_mode: GenMode) -> String {
        let mut modules = modules
            .iter()
            .map(|r#mod| {
                if gen_mode == GenMode::Types && r#mod.submodules.is_empty() {
                    let postfix = "_types";
                    format!("pub mod {}{};", r#mod.name.to_snake_case(), postfix)
                } else {
                    format!("pub mod {};", r#mod.name.to_snake_case())
                }
            })
            .collect::<Vec<_>>();
        modules.sort_unstable();
        modules.join("\n")
    }

    fn build_types(&self) -> Result<String> {
        let mut methods: Vec<String> = Vec::with_capacity(self.methods.len());
        for method in &self.methods {
            let method = format!(
                "{request}
            
                {response}
    
                {error}
                ",
                request = method.build_request_type()?,
                response = method.build_response_type()?,
                error = method.build_error_type()?
            );
            methods.push(method);
        }
        Ok(methods.join("\n"))
    }

    fn build_calls(&self, gen_mode: GenMode) -> Result<String> {
        let mut methods: Vec<String> = Vec::with_capacity(self.methods.len());
        for method in &self.methods {
            methods.push(method.build_call_method(gen_mode)?);
        }
        Ok(methods.join("\n"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GenMode {
    Async,
    Sync,
    Types,
}

impl GenMode {
    fn import(self) -> &'static str {
        match self {
            GenMode::Async => "use crate::async_impl::SlackWebRequestSender;",
            GenMode::Sync => "use crate::sync::SlackWebRequestSender;",
            _ => unreachable!(),
        }
    }

    fn dot_await(self) -> &'static str {
        match self {
            GenMode::Async => ".await",
            GenMode::Sync => "",
            _ => unreachable!(),
        }
    }
    fn fn_type(self) -> &'static str {
        match self {
            GenMode::Async => "async fn",
            GenMode::Sync => "fn",
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub documentation_url: String,
    pub parameters: Vec<Parameter>,
    pub response: Response,
    pub http_method: HttpMethod,
    pub errors: Vec<String>,
}

impl Method {
    fn build_request_type(&self) -> Result<String> {
        let type_prefix = self.name.to_pascal_case();
        let struct_name = format!("{}Request", type_prefix);
        let lifetime = if self
            .parameters
            .iter()
            .filter(|p| p.name != "token")
            .any(|p| p.param_type == ParameterDataType::String)
        {
            "<'a>"
        } else {
            ""
        };
        let parameters = self
            .parameters
            .iter()
            .filter(|p| p.name != "token")
            .map(Parameter::to_rust)
            .collect::<Vec<_>>()
            .join("\n");
        let out = format!(
            "#[derive(Clone, Default, Debug)]
            pub struct {name}{lifetime} {{
                {parameters}
            }}",
            name = struct_name,
            parameters = parameters,
            lifetime = lifetime,
        );
        Ok(out)
    }

    fn build_response_type(&self) -> Result<String> {
        let type_prefix = self.name.to_pascal_case();
        let struct_name = format!("{}Response", type_prefix);
        let struct_name_error = format!("{}Error", type_prefix);
        let (_, r#types) = self.response.to_rust(
            &self.name.to_pascal_case(),
            Some(&struct_name),
            &struct_name_error,
            true,
            &mut HashSet::new(),
        )?;
        let out = r#types.join("\n\n");
        Ok(out)
    }

    fn build_error_type(&self) -> Result<String> {
        let type_prefix = self.name.to_pascal_case();
        let struct_name = format!("{}Error", type_prefix);
        let mut custom_errors = self
            .errors
            .iter()
            .map(|e| e.to_pascal_case())
            .collect::<Vec<_>>()
            .join(",\n");
        if !custom_errors.is_empty() {
            custom_errors.push(',');
            custom_errors.push('\n');
        }
        let mut custom_errors_from = self
            .errors
            .iter()
            .map(|e| format!("\"{}\" => {}::{}", e, struct_name, e.to_pascal_case()))
            .collect::<Vec<_>>()
            .join(",\n");
        if !custom_errors_from.is_empty() {
            custom_errors_from.push(',');
            custom_errors_from.push('\n');
        }
        let mut custom_errors_dis = self
            .errors
            .iter()
            .map(|e| {
                format!(
                    "{}::{} => write!(f, \"Server returned error {}\")",
                    struct_name,
                    e.to_pascal_case(),
                    e
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        if !custom_errors_dis.is_empty() {
            custom_errors_dis.push(',');
            custom_errors_dis.push('\n');
        }
        let out = format!(
            "#[derive(Debug)]
            pub enum {name}<E: Error> {{
                {custom_errors}/// The response was not parseable as the expected object
                MalformedResponse(String, serde_json::error::Error),
                /// The response returned an error that was unknown to the library
                Unknown(String),
                /// The client had an error sending the request to Slack
                Client(E),
            }}
            
            impl<'a, E: Error> From<&'a str> for {name}<E> {{
                fn from(s: &'a str) -> Self {{
                    match s {{
                        {custom_errors_from}_ => {name}::Unknown(s.to_owned()),
                    }}
                }}
            }}

            impl<E: Error> fmt::Display for {name}<E> {{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                    match *self {{
                        {custom_errors_dis}{name}::MalformedResponse(_, ref e) => write!(f, \"{{}}\", e),
                        {name}::Unknown(ref s) => write!(f, \"{{}}\", s),
                        {name}::Client(ref inner) => write!(f, \"{{}}\", inner),
                    }}
                }}
            }}
            
            impl<E: Error + 'static> Error for {name}<E> {{
                fn source(&self) -> Option<&(dyn Error + 'static)> {{
                    match *self {{
                        {name}::MalformedResponse(_, ref e) => Some(e),
                        {name}::Client(ref inner) => Some(inner),
                        _ => None,
                    }}
                }}
            }}",
            name = struct_name,
            custom_errors = custom_errors,
            custom_errors_from = custom_errors_from,
            custom_errors_dis = custom_errors_dis,
        );
        Ok(out)
    }

    fn build_call_method(&self, gen_mode: GenMode) -> Result<String> {
        let type_prefix = self.name.to_pascal_case();
        let req_lt = if self
            .parameters
            .iter()
            .filter(|p| p.name != "token")
            .any(|p| p.param_type == ParameterDataType::String)
        {
            "<'_>"
        } else {
            ""
        };
        let request_type = format!("{}Request{}", type_prefix, req_lt);
        let response_type = format!("{}Response", type_prefix);
        let error_type = format!("{}Error", type_prefix);
        let fn_name = self.name.to_snake_case();
        let parameter_converts = self
            .parameters
            .iter()
            .filter(|p| p.name != "token")
            .filter(|p| p.param_type != ParameterDataType::String)
            .map(Parameter::to_rust_fn_convert)
            .collect::<Vec<_>>()
            .join("\n");
        let parameters = self
            .parameters
            .iter()
            .filter(|p| p.name != "token")
            .map(Parameter::to_rust_fn)
            .collect::<Vec<_>>()
            .join("\n");
        let token = self.parameters.iter().find(|p| p.name == "token");
        let headers = match self.http_method {
            HttpMethod::Get => "",
            HttpMethod::Post => {
                if let Some(token) = token {
                    if token.required {
                        ", &[(\"token\", token)]"
                    } else {
                        ", &token.map_or(vec![], |t| vec![(\"token\", t)])"
                    }
                } else {
                    ", &[]"
                }
            }
        };
        let params = match self.http_method {
            HttpMethod::Post => "",
            HttpMethod::Get => {
                if let Some(token) = token {
                    if token.required {
                        "Some((\"token\", token)),"
                    } else {
                        "token.map(|token| (\"token\", token)),"
                    }
                } else {
                    ""
                }
            }
        };
        let token_param = if let Some(token) = token {
            if token.required {
                "token: &str,"
            } else {
                "token: Option<&str>,"
            }
        } else {
            ""
        };
        let empty_param = if parameters.is_empty() { "_" } else { "" };
        let out = format!(
            "/// {description}
            ///
            /// Wraps {doc_url}
            
            pub {fn_type} {fn_name}<R>(
                client: &R,{token_param}
                {empty_param}request: &{request_type},
            ) -> Result<{response_type}, {error_type}<R::Error>>
            where
                R: SlackWebRequestSender,
            {{
                {parameter_converts}
                let params: Vec<Option<(&str, &str)>> = vec![
                    {params}{parameters}
                ];
                let params: Vec<(&str, &str)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
                let url = crate::get_slack_url_for_method(\"{full_name}\");
                client
                    .{method}(&url, &params[..]{headers}){dot_await}
                    .map_err({error_type}::Client)
                    .and_then(|result| {{
                        serde_json::from_str::<{response_type}>(&result)
                            .map_err(|e| {error_type}::MalformedResponse(result, e))
                    }})
                    .and_then(|o| o.into())
            }}",
            description = self.description,
            doc_url = self.documentation_url,
            request_type = request_type,
            response_type = response_type,
            error_type = error_type,
            fn_name = fn_name,
            full_name = self.full_name,
            fn_type = gen_mode.fn_type(),
            dot_await = gen_mode.dot_await(),
            parameters = parameters,
            method=self.http_method.method(),
            headers=headers,
            token_param=token_param,
            params = params,
            empty_param = empty_param,
            parameter_converts = parameter_converts,
        );
        Ok(out)
    }
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub description: Option<String>,
    pub name: String,
    pub required: bool,
    pub param_type: ParameterDataType,
}

impl Parameter {
    fn to_rust(&self) -> String {
        let description = self.description_to_rust();
        let r#type = self.param_type.to_rust(self.required);
        format!("{description}pub {name}: {type},", description = description, name=self.name, type=r#type)
    }

    fn to_rust_fn_convert(&self) -> String {
        if self.param_type == ParameterDataType::String {
            panic!("Not required for string types");
        } else if self.required {
            format!(
                "let {name}: Option<Cow<'_, str>> = Some(request.{name}.to_string().into());",
                name = self.name
            )
        } else {
            format!(
                    "let {name}: Option<Cow<'_, str>> = request.{name}.as_ref().map(|{name}| {name}.to_string().into());",
                    name = self.name
                )
        }
    }

    fn to_rust_fn(&self) -> String {
        if self.param_type == ParameterDataType::String {
            if self.required {
                format!(
                    "Some((\"{name}\", request.{name}.as_ref())),",
                    name = self.name
                )
            } else {
                format!(
                    "request.{name}.as_ref().map(|{name}| (\"{name}\", {name}.as_ref())),",
                    name = self.name
                )
            }
        } else {
            format!(
                "{name}.as_ref().map(|{name}| (\"{name}\", {name}.as_ref())),",
                name = self.name
            )
        }
    }

    fn description_to_rust(&self) -> String {
        self.description
            .as_ref()
            .map(|p| {
                let p = p.trim();
                format!("/// {}\n", p)
            })
            .unwrap_or_else(String::new)
    }
}

impl TryFrom<&schema::Parameter> for Parameter {
    type Error = anyhow::Error;

    fn try_from(value: &schema::Parameter) -> Result<Self, Self::Error> {
        Ok(Parameter {
            description: value.description.clone(),
            name: value.name.clone(),
            required: value.required.unwrap_or(false),
            param_type: ParameterDataType::from_str(&value.param_type)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterDataType {
    Bool,
    Decimal,
    Int,
    String,
}

impl ParameterDataType {
    pub fn to_rust(&self, required: bool) -> String {
        let r#type = match self {
            Self::Bool => "bool",
            Self::Decimal => "f64",
            Self::Int => "u64",
            Self::String => "Cow<'a, str>",
        };
        if required {
            r#type.to_string()
        } else {
            format!("Option<{}>", r#type)
        }
    }
}

impl FromStr for ParameterDataType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r#type = match s {
            "boolean" => Self::Bool,
            "integer" => Self::Int,
            "number" => Self::Decimal,
            "string" => Self::String,
            t => bail!(format!("Type {} currently not supported", t)),
        };
        Ok(r#type)
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub description: Option<String>,
    pub r#type: ResponseType,
    pub required: bool,
}

impl Response {
    pub fn to_rust(
        &self,
        method_name: &str,
        name: Option<&str>,
        error_name: &str,
        top: bool,
        names: &mut HashSet<String>,
    ) -> Result<(String, Vec<String>)> {
        let res = match &self.r#type {
            // Inner
            ResponseType::Bool if !top => ("bool".into(), Vec::new()),
            ResponseType::Decimal if !top => ("f64".into(), Vec::new()),
            ResponseType::Int if !top => ("u64".into(), Vec::new()),
            ResponseType::String if !top => ("String".into(), Vec::new()),
            ResponseType::RawJson if !top => ("serde_json::Value".into(), Vec::new()),
            ResponseType::Vec(res) if !top => {
                let (r#type, types) = res.to_rust(method_name, name, error_name, false, names)?;
                (format!("Vec<{}>", r#type), types)
            }

            // Top
            ResponseType::Vec(res) => Response {
                description: None,
                r#type: ResponseType::Object(vec![Member {
                    name: "result".into(),
                    r#type: *res.clone(),
                }]),
                required: true,
            }
            .to_rust(method_name, name, error_name, true, names)?,
            ResponseType::Object(mem) => {
                let mut members: Vec<String> = Vec::with_capacity(mem.len());
                let mut types = Vec::new();
                for member in mem {
                    let (name, types_mem) = member.to_rust(method_name, error_name, names)?;
                    types.extend_from_slice(&types_mem);
                    members.push(name)
                }
                let name = name.map_or_else(|| format!("{}List", method_name), |s| s.into());
                let mut struct_name = if !top {
                    format!("{}Inner", name)
                } else {
                    name.clone()
                };
                let mut i = 1;
                while names.contains(&struct_name) {
                    struct_name = format!("{}{}Inner", name, i);
                    i += 1;
                }
                names.insert(struct_name.clone());

                let out = format!(
                    "#[derive(Clone, Debug, Deserialize)]
                    pub struct {name} {{
                        {members}
                    }}",
                    name = struct_name,
                    members = members.join("\n")
                );
                types.push(out);
                if top {
                    let error_handling = if mem.iter().any(|m| m.name == "error") {
                        "Err(self.error.as_ref().map(String::as_ref).unwrap_or(\"\").into())".into()
                    } else {
                        format!("Err({}::Unknown(\"Server failed without providing an error message.\".into()))", error_name)
                    };
                    let var_ok = if mem.iter().any(|m| m.name == "ok") {
                        "ok"
                    } else {
                        "result.ok"
                    };
                    let out = format!(
                        "impl<E: Error> Into<Result<{name}, {error_name}<E>>> for {name} {{
                            fn into(self) -> Result<{name}, {error_name}<E>> {{
                                if self.{var_ok} {{
                                    Ok(self)
                                }} else {{
                                    {error_handling}
                                }}
                            }}
                        }}",
                        name = name,
                        error_name = error_name,
                        error_handling = error_handling,
                        var_ok = var_ok,
                    );
                    types.push(out);
                }
                (struct_name, types)
            }
            t => bail!("Object {:?}, is not supported at position top = {}", t, top),
        };
        Ok(res)
    }
}

impl TryFrom<&schema::Schema> for Response {
    type Error = anyhow::Error;

    fn try_from(s: &schema::Schema) -> Result<Self, Self::Error> {
        Ok(Self {
            description: s
                .title
                .as_ref()
                .or_else(|| s.description.as_ref())
                .map(Clone::clone),
            r#type: ResponseType::try_from(s)?,
            required: false,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub r#type: Response,
}

impl Member {
    pub fn to_rust(
        &self,
        method_name: &str,
        error_name: &str,
        names: &mut HashSet<String>,
    ) -> Result<(String, Vec<String>)> {
        let name = format!("{}{}", method_name, self.name.to_pascal_case());
        let (mut r#type, types) =
            self.r#type
                .to_rust(method_name, Some(&name), error_name, false, names)?;
        if !self.r#type.required && self.name != "ok" {
            r#type = format!("Option<{}>", r#type);
        }
        let name = match self.name.as_ref() {
            "self" => "r#_self",
            "type" => "r#type",
            v => v,
        };
        let r#pub = match name {
            "ok" => "#[serde(default)]\n",
            "error" => "",
            _ => "pub ",
        };
        Ok((
            format!("{pub}{name}: {type},", pub=r#pub, name=name, type=r#type),
            types,
        ))
    }
}

impl TryFrom<(&str, &schema::Schema)> for Member {
    type Error = anyhow::Error;

    fn try_from(s: (&str, &schema::Schema)) -> Result<Self, Self::Error> {
        Ok(Self {
            name: s.0.into(),
            r#type: Response::try_from(s.1)
                .with_context(|| format!("Unable to convert type in object {}", s.0))?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum ResponseType {
    Bool,
    Decimal,
    Int,
    String,
    Object(Vec<Member>),
    RawJson,
    Vec(Box<Response>),
}

impl TryFrom<&schema::Schema> for ResponseType {
    type Error = anyhow::Error;

    fn try_from(s: &schema::Schema) -> Result<Self, Self::Error> {
        let schema: Option<Vec<&str>> = s
            .schema_type
            .as_ref()
            .map(|v| v.iter().map(AsRef::as_ref).collect());
        let items = s.items.as_ref().and_then(|s| {
            s.iter().find(|s| {
                s.schema_type
                    .as_ref()
                    .map_or(true, |s| s.iter().any(|s| s != "null"))
            })
        });
        let properties = s.properties.as_ref();

        let r#type = match (schema, items, properties) {
            // Primitives
            (Some(schema), _, _) if schema.contains(&"boolean") => Self::Bool,
            (Some(schema), _, _) if schema.contains(&"integer") => Self::Int,
            (Some(schema), _, _) if schema.contains(&"number") => Self::Decimal,
            (Some(schema), _, _) if schema.contains(&"string") => Self::String,

            // Object
            (Some(schema), _, Some(properties)) if schema.contains(&"object") => {
                let mut members = Vec::with_capacity(properties.len());
                for (name, sub_schema) in properties {
                    let mut member = Member::try_from((name.as_ref(), sub_schema))?;
                    if s.required
                        .as_ref()
                        .map_or(false, |v| v.contains(&member.name))
                    {
                        member.r#type.required = true;
                    }
                    members.push(member);
                }
                ResponseType::Object(members)
            }

            // Array
            (Some(schema), Some(item), _) if schema.contains(&"array") => {
                let mut res = Response::try_from(item)?;
                res.required = true;
                ResponseType::Vec(Box::new(res))
            }
            (None, Some(item), _) => {
                let mut res = Response::try_from(item)?;
                res.required = true;
                ResponseType::Vec(Box::new(res))
            }

            // RawJson
            (Some(schema), None, None) if schema.contains(&"object") => Self::RawJson,
            (Some(schema), None, None) if schema.contains(&"null") => Self::RawJson,
            (None, None, None) => Self::RawJson,

            // Other
            t => bail!(format!("Type {:?} currently not supported", t)),
        };
        Ok(r#type)
    }
}

#[derive(Clone, Debug, Copy)]
pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    fn method(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
        }
    }
}
