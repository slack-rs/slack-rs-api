use crate::rust::{Method, Module};
use anyhow::{bail, Context, Result};
use inflector::Inflector;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

pub fn create_adapt_skeleton<P: AsRef<Path>>(outdir: P, modules: &[Module]) -> Result<()> {
    let outdir = outdir.as_ref();
    create_dir(outdir)?;
    for module in modules {
        generate_module(&outdir, module)?;
    }
    let outdir = outdir.join("mod.rs");
    if outdir.exists() {
        return Ok(());
    }
    let mods = generate_mods(modules);
    let correct = generate_correct(modules, &[], true);
    write_file(
        &outdir,
        &format!(
            "#![allow(unused_imports)]
            #![allow(clippy::single_match)]
    use crate::rust::{{Method, Module}};
    use crate::adapt::utils::*;
    
    {}
    
    {}",
            mods, correct
        ),
    )?;
    Ok(())
}

pub fn generate_module(outdir: &Path, module: &Module) -> Result<()> {
    let modules = generate_mods(&module.submodules);
    let mod_space = if !module.submodules.is_empty() {
        "\n\n"
    } else {
        ""
    };
    let method_space = if !module.methods.is_empty() {
        "\n\n"
    } else {
        ""
    };
    let correct = generate_correct(&module.submodules, &module.methods, false);
    let methods = generate_methods(&module.methods);

    let file_path = if module.submodules.is_empty() {
        outdir.join(&format!("{}.rs", module.name.to_snake_case()))
    } else {
        let outdir = outdir.join(&module.name.to_snake_case());
        create_dir(&outdir)?;
        for module in &module.submodules {
            generate_module(&outdir, module)?;
        }
        outdir.join("mod.rs")
    };
    if file_path.exists() {
        return Ok(());
    }

    write_file(
        &file_path,
        &format!(
            "#![allow(unused_imports)]
            #![allow(clippy::single_match)]
            use crate::rust::{{Method, Module}};
            use crate::adapt::utils::*;

            {modules}{mod_space}{correct}{method_space}{methods}",
            modules = modules,
            mod_space = mod_space,
            methods = methods,
            correct = correct,
            method_space = method_space,
        ),
    )?;
    Ok(())
}

fn generate_mods(modules: &[Module]) -> String {
    let mut modules = modules
        .iter()
        .map(|m| format!("mod {};", m.name.to_snake_case()))
        .collect::<Vec<_>>();
    modules.sort_unstable();
    modules.join("\n")
}

fn generate_correct(modules: &[Module], methods: &[Method], top: bool) -> String {
    let param = if top {
        "modules: &mut Vec<Module>"
    } else {
        "module: &mut Module"
    };
    let modules = modules
        .iter()
        .map(|m| {
            format!(
                "\"{name}\" => {sname}::correct(&mut module),",
                name = m.name,
                sname = m.name.to_snake_case()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let modules = if modules.is_empty() {
        String::new()
    } else {
        let modules = format!(
            r#"match module.name.as_str() {{
                {}
                _ => {{}},
            }}"#,
            modules
        );
        if top {
            format!(
                r#"for mut module in modules {{
                    {}
                }}"#,
                modules
            )
        } else {
            format!(
                r#"for mut module in &mut module.submodules {{
                   {}   
                }}
                
                "#,
                modules
            )
        }
    };
    let methods = methods
        .iter()
        .map(|m| {
            format!(
                "\"{name}\" => correct_{sname}(&mut method),",
                name = m.name,
                sname = m.name.to_snake_case()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let methods = if methods.is_empty() {
        String::new()
    } else {
        format!(
            r#"for mut method in &mut module.methods {{
                match method.name.as_str() {{
                    {}
                    _ => {{}},
                }}
            }}"#,
            methods
        )
    };
    format!(
        r#"pub fn correct({param}) {{
            {modules}{methods}
        }}"#,
        param = param,
        modules = modules,
        methods = methods,
    )
}

fn generate_methods(methods: &[Method]) -> String {
    methods
        .iter()
        .map(|m| generate_method(m))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn generate_method(method: &Method) -> String {
    format!(
        "fn correct_{name}(_method: &mut Method) {{}}",
        name = method.name.to_snake_case()
    )
}

fn create_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Unable to create directory at: {:?}", path))?;
    }
    Ok(())
}

fn write_file(path: &Path, data: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .with_context(|| format!("Unable to create file at: {:?}", path))?;

    file.write_all(data.as_bytes())
        .with_context(|| format!("Failed to write to file: {:?}", path))?;

    let out = Command::new("rustfmt")
        .args(&["--edition", "2018"])
        .arg(&path)
        .output()
        .context("Unable to run rustfmt")?;
    if !out.status.success() {
        bail!("rustfmt failed for {:?}", path);
    }
    Ok(())
}
