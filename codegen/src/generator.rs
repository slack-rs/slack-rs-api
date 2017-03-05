use inflector::Inflector;

use json_schema::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Module {
    pub name: String,
    pub description: Option<String>,
    pub methods: Vec<Method>,
}

impl Module {
    pub fn generate(&self) -> String {
        format!(
            "{docs}

            #[allow(unused_imports)]
            use std::collections::HashMap;
            use std::convert::From;
            use std::error::Error;
            use std::fmt;

            use serde_json;

            use ::requests::SlackWebRequestSender;

            {methods}",
            docs = self.description.as_ref().map(|d| format_docs("//!", &d)).unwrap_or(String::new()),
            methods = self.methods
                .iter()
                .map(Method::generate)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    pub fn get_safe_name(&self) -> String {
        self.name.replace('.', "_")
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Method {
    pub name: String,
    pub description: String,
    #[serde(rename = "documentationUrl")]
    pub documentation_url: String,
    pub params: Vec<Param>,
    pub response: Response,
}

impl Method {
    pub fn generate(&self) -> String {
        // HACK: files.upload requires multipart support, which is not yet supported by this library
        if self.name == "files.upload" {
            return String::new();
        }

        let fn_name = self.name.split('.').last().unwrap().to_snake_case();
        let type_prefix = self.name.split('.').last().unwrap().to_pascal_case();
        let request_struct_name = type_prefix.clone() + "Request";
        let response_struct_name = type_prefix.clone() + "Response";
        let error_enum_name = type_prefix.clone() + "Error";
        let response = self.response.generate(&response_struct_name, &error_enum_name);
        let response_type = self.response.get_response_type(&response_struct_name);

        let send_call = {
            let mut base_call = format!("client.send(\"{name}\", &params[..])
                .map_err(|err| {error_type}::Client(err))
                .and_then(|result| {{
                    serde_json::from_str::<{response_type}>(&result)
                        .map_err(|e| {error_type}::MalformedResponse(e))
                }})",
                name = self.name,
                response_type = response_struct_name,
                error_type = error_enum_name
            );

            match response_type {
                PropType::Obj(ref o) => if o.has_ok() { base_call.push_str(".and_then(|o| o.into())") },
                PropType::Enum(ref e) => if e.has_ok() { base_call.push_str(".and_then(|o| o.into())") },
                _ => panic!("Top-level response for {} is not an object or enum.", fn_name)
            }

            base_call
        };

        if self.params.is_empty() {
            format!(
                "{documentation}
                pub fn {method_name}<R>(client: &R) -> Result<{response_type}, {error_type}<R::Error>>
                    where R: SlackWebRequestSender
                {{
                    let params = &[];
                    {send_call}
                }}

                {response}
                ",
                documentation = format_docs("///", &[
                    &self.description,
                    "",
                    &format!("Wraps {}", self.documentation_url)
                ].join("\n")),
                method_name = fn_name,
                response_type = response_struct_name,
                error_type = error_enum_name,
                response = response,
                send_call = send_call
            )
        } else if self.params.len() == 1 && self.params[0].ty == "auth_token" {
            format!(
                "{documentation}
                pub fn {method_name}<R>(client: &R, token: &str) -> Result<{response_type}, {error_type}<R::Error>>
                    where R: SlackWebRequestSender
                {{
                    let params = &[(\"token\", token)];
                    {send_call}
                }}

                {response}
                ",
                documentation = format_docs("///", &[
                    &self.description,
                    "",
                    &format!("Wraps {}", self.documentation_url)
                ].join("\n")),
                method_name = fn_name,
                response_type = response_struct_name,
                error_type = error_enum_name,
                response = response,
                send_call = send_call
            )
        } else {
            let has_token = self.params.iter().find(|p| p.ty == "auth_token").is_some();
            let method_params = if has_token {
                format!("client: &R, token: &str, request: &{}", request_struct_name)
            } else {
                format!("client: &R, request: &{}", request_struct_name)
            };
            format!(
                "{documentation}
                pub fn {method_name}<R>({method_params}) -> Result<{response_type}, {error_type}<R::Error>>
                    where R: SlackWebRequestSender
                {{
                    {local_vars}
                    let params = vec![
                        {token}
                        {param_pairs}
                    ];
                    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
                    {send_call}
                }}

                {request}

                {response}
                ",
                documentation = format_docs("///", &[
                    &self.description,
                    "",
                    &format!("Wraps {}", self.documentation_url)
                ].join("\n")),
                method_name = fn_name,
                response_type = response_struct_name,
                error_type = error_enum_name,
                response = response,
                request = self.get_request_struct(&request_struct_name),
                method_params = method_params,
                token = if has_token { "Some((\"token\", token))," } else { "" },
                local_vars = self.params.iter()
                    .filter(|p| p.ty != "auth_token") // passed in method params instead
                    .filter(|p| p.name != "simple_latest") // HACK: simple_latest breaks deserialization
                    .filter_map(|p| p.lifted())
                    .collect::<Vec<_>>()
                    .join("\n"),
                param_pairs = self.params.iter()
                    .filter(|p| p.ty != "auth_token") // passed in method params instead
                    .filter(|p| p.name != "simple_latest") // HACK: simple_latest breaks deserialization
                    .map(Param::get_pair)
                    .collect::<Vec<String>>()
                    .join(",\n"),
                send_call = send_call
            )
        }
    }

    fn get_request_struct(&self, ty_name: &str) -> String {
        format!(
            "#[derive(Clone, Default, Debug)]
            pub struct {request_type}{lifetime} {{
                {request_params}
            }}",
            request_type = ty_name,
            request_params = self.params.iter()
                .filter(|p| p.ty != "auth_token") // passed in method params instead
                .filter(|p| p.name != "simple_latest") // HACK: simple_latest breaks deserialization
                .map(Param::generate).collect::<Vec<String>>().join("\n"),
            lifetime = if self.params.iter()
                .filter(|p| p.ty != "auth_token")
                .all(|p| p.ty == "integer" || p.ty == "boolean") { "" } else { "<'a>" }
        )
    }
}

pub trait Okable {
    fn has_ok(&self) -> bool;
}

impl Okable for JsonObject {
    fn has_ok(&self) -> bool {
        self.fields.iter().find(|f| f.name == "ok").is_some()
    }
}

impl Okable for JsonEnum {
    fn has_ok(&self) -> bool {
        self.variants.iter().all(|v| match &v.inner {
            &PropType::Obj(ref o) => o.has_ok(),
            &PropType::Enum(ref e) => e.has_ok(),
            _ => false
        })
    }
}

fn generate_matches<F>(enm: &JsonEnum, var_name: &str, f: F) -> Vec<String>
    where F: Fn(&JsonEnumVariant) -> String
{
    enm.variants
        .iter()
        .map(|v| {
            format!(
                "{variant}({var_name}) => {body},",
                variant = v.qualified_name,
                var_name = var_name,
                body = f(&v)
            )
        })
        .collect()
}

fn get_obj_to_response_impl(obj: &JsonObject, error_type: &str) -> Option<String> {
    if obj.has_ok() {
        Some(format!(
            "impl<E: Error> Into<Result<{name}, {error_ty}<E>>> for {name} {{
                fn into(self) -> Result<{name}, {error_ty}<E>> {{
                    if self.ok {{
                        Ok(self)
                    }} else {{
                        Err(self.error.as_ref().map(String::as_ref).unwrap_or(\"\").into())
                    }}
                }}
            }}",
            error_ty = error_type,
            name = obj.name
        ))
    } else {
        None
    }
}

fn get_enum_to_response_impl(enm: &JsonEnum, error_type: &str) -> Option<String> {
    if enm.has_ok() {
        Some(format!(
            "impl<E: Error> Into<Result<{name}, {error_ty}<E>>> for {name} {{
                fn into(self) -> Result<{name}, {error_ty}<E>> {{
                    match self {{
                        {matches}
                    }}
                }}
            }}
            
            {inner_impls}",
            error_ty = error_type,
            name = enm.name,
            matches = generate_matches(enm, "inner", |v| {
                format!("{{ let x: Result<{}, {}<E>> = inner.into(); x.map(|r| {}(r)) }}", enm.name.clone() + &v.name, error_type, v.qualified_name)
            }).join("\n"),
            inner_impls = enm.variants.iter()
                .map(|v| match &v.inner {
                    &PropType::Obj(ref o) => get_obj_to_response_impl(o, error_type).expect("Top-level enum inner object did not have \"ok\" field."),
                    &PropType::Enum(ref e) => get_enum_to_response_impl(e, error_type).expect("Top-level enum inner variant did not have \"ok\" field."),
                    _ => panic!("Top-level enum is does not contain a type that can have an \"ok\" field.")
                })
                .collect::<Vec<_>>()
                .join("\n")
        ))
    } else {
        None
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    pub sample: String,
    pub schema: JsonSchema,
    pub errors: Vec<ApiError>,
}

impl Response {
    pub fn generate(&self, ty_name: &str, error_ty: &str) -> String {
        let (objs, to_result) = match PropType::from_schema(&self.schema, ty_name) {
            PropType::Obj(ref o) => {
                let to_result = get_obj_to_response_impl(o, error_ty);
                (o.to_code(), to_result)
            },
            PropType::Enum(ref e) => {
                let to_result = get_enum_to_response_impl(e, error_ty);
                (e.to_code(), to_result)
            },
            _ => {
                panic!("Top level response schema for {} is not an object or enum. {:?}",
                       ty_name,
                       self.schema)
            }
        };
        format!(
            "{objs}
            {slack_result}
            {errors}",
            objs = objs,
            slack_result = to_result.unwrap_or("".into()),
            errors = self.get_error_enum(error_ty),
        )
    }

    pub fn get_response_type(&self, ty_name: &str) -> PropType {
        PropType::from_schema(&self.schema, ty_name)
    }

    fn get_error_enum(&self, error_ty: &str) -> String {
        format!(
            "#[derive(Debug)]
            pub enum {error_type}<E: Error> {{
                {variants}
                /// The response was not parseable as the expected object
                MalformedResponse(serde_json::error::Error),
                /// The response returned an error that was unknown to the library
                Unknown(String),
                /// The client had an error sending the request to Slack
                Client(E)
            }}
            
            impl<'a, E: Error> From<&'a str> for {error_type}<E> {{
                fn from(s: &'a str) -> Self {{
                    match s {{
                        {matches}
                        _ => {error_type}::Unknown(s.to_owned())
                    }}
                }}
            }}

            impl<E: Error> fmt::Display for {error_type}<E> {{
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                     write!(f, \"{{}}\", self.description())
                }}
            }}
            
            impl<E: Error> Error for {error_type}<E> {{
                fn description(&self) -> &str {{
                    match self {{
                        {description_matches}
                        &{error_type}::MalformedResponse(ref e) => e.description(),
                        &{error_type}::Unknown(ref s) => s,
                        &{error_type}::Client(ref inner) => inner.description()
                    }}
                }}

                fn cause(&self) -> Option<&Error> {{
                    match self {{
                        &{error_type}::MalformedResponse(ref e) => Some(e),
                        &{error_type}::Client(ref inner) => Some(inner),
                        _ => None
                    }}
                }}
            }}",
            error_type = error_ty,
            variants = self.errors
                .iter()
                .map(|e| {
                    format!(
                        "{docs}\n{name},",
                        docs = format_docs("///", &e.description),
                        name = e.name.to_pascal_case()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            matches = self.errors
                .iter()
                .map(|e| {
                    format!(
                        "\"{str_name}\" => {error_ty}::{ty_name},",
                        error_ty = error_ty,
                        str_name = e.name,
                        ty_name = e.name.to_pascal_case()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            description_matches = self.errors
                .iter()
                .map(|e| {
                    format!(
                        "&{error_ty}::{ty_name} => \"{str_name}: {description}\",",
                        error_ty = error_ty,
                        str_name = e.name,
                        description = e.description,
                        ty_name = e.name.to_pascal_case()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Param {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub optional: bool,
}

impl Param {
    fn generate(&self) -> String {
        format!(
            "{documentation}\npub {name}: {ty},",
            documentation = format_docs("///", &self.description),
            name = self.name,
            ty = self.get_rust_type()
        )
    }

    pub fn lifted(&self) -> Option<String> {
        match (&self.ty[..], self.optional) {
            ("integer", true) => Some(format!("let {name} = request.{name}.map(|{name}| {name}.to_string());", name = self.name)),
            ("integer", false) => Some(format!("let {name} = request.{name}.to_string();", name = self.name)),
            _ => None
        }
    }

    pub fn get_pair(&self) -> String {
        match (&self.ty[..], self.optional) {
            ("boolean", true) => {
                format!("request.{name}.map(|{name}| (\"{name}\", if {name} {{ \"1\" }} else {{ \"0\" }}))", name = self.name)
            },
            ("boolean", false) => {
                format!("Some((\"{name}\", if request.{name} {{ \"1\" }} else {{ \"0\" }}))", name = self.name)
            },
            ("integer", true) => {
                // lifted into local variable, using {name} instead of request.{name}
                format!("{name}.as_ref().map(|{name}| (\"{name}\", &{name}[..]))", name = self.name)
            },
            ("integer", false) => {
                // lifted into local variable, using {name} instead of request.{name}
                format!("Some((\"{name}\", &{name}[..]))", name = self.name)
            },
            (_, true) => {
                format!("request.{name}.map(|{name}| (\"{name}\", {name}))", name = self.name)
            },
            (_, false) => {
                format!("Some((\"{name}\", request.{name}))", name = self.name)
            }
        }
    }

    fn get_rust_type(&self) -> String {
        let ty = match &self.ty[..] {
            "boolean" => "bool",
            "integer" => "u32",
            _ => "&'a str",
        };
        if self.optional {
            return format!("Option<{}>", ty);
        } else {
            return ty.to_owned();
        }
    }
}

impl JsonObjectFieldInfo {
    pub fn to_code(&self) -> String {
        let mut prefix = String::new();
        if self.name == "ok" {
            prefix.push_str("#[serde(default)]");
        } else if self.name != "error" && self.name != "ok" {
            prefix.push_str("pub");
        };
        if let Some(ref rename) = self.rename {
            format!(
                "#[serde(rename = \"{}\")]\n{} {}: {},",
                rename,
                prefix,
                self.name,
                self.ty.to_rs_type()
            )
        } else {
            format!("{} {}: {},", prefix, self.name, self.ty.to_rs_type())
        }
    }
}

impl JsonEnumVariant {
    pub fn to_code(&self) -> String {
        format!(
            "{name}({inner}),",
            name = self.name,
            inner = self.inner.to_rs_type()
        )
    }
}

impl JsonEnum {
    pub fn to_code(&self) -> String {
        // Hack to work around message having a different identifier here
        let (variant_field, on_missing_field) = if self.name == "Message" {
            ("subtype", "::serde_json::from_value::<MessageStandard>(value.clone()).map(|obj| {{
                Message::Standard(obj)
            }}).map_err(|e| D::Error::custom(&format!(\"{}\", e)))")
        } else {
            ("type", "Err(D::Error::missing_field(\"type\"))")
        };
        
        let mut subobjs = self.variants.clone();

        subobjs.sort_by_key(|v| v.name.clone());

        let subobjs = subobjs.iter()
            .flat_map(|v| obj_recur(&v.inner))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "
            #[derive(Clone, Debug)]
            pub enum {name} {{
                {variants}
            }}

            impl ::serde::Deserialize for {name} {{
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: ::serde::Deserializer
                {{
                    use ::serde::de::Error as SerdeError;

                    const VARIANTS: &'static [&'static str] = &[{variant_names}];

                    let value = ::serde_json::Value::deserialize(deserializer)?;
                    if let Some(ty_val) = value.get(\"{variant_field}\") {{
                        if let Some(ty) = ty_val.as_str() {{
                            match ty {{
                                {variant_matches}
                                _ => Err(D::Error::unknown_variant(ty, VARIANTS))
                            }}
                        }} else {{
                            Err(D::Error::invalid_type(::serde::de::Unexpected::Unit, &\"a string\"))
                        }}
                    }} else {{
                        {on_missing_field}
                    }}
                }}
            }}

            {subobjs}",
            name = self.name,
            variants = self.variants
                .iter()
                .map(|v| v.to_code())
                .collect::<Vec<_>>()
                .join("\n"),
            variant_names = self.variants
                .iter()
                .map(|v| format!("\"{}\"", v.name.to_snake_case()))
                .collect::<Vec<_>>()
                .join(","),
            variant_matches = self.variants
                .iter()
                .map(|v| format!(
                    "\"{type_name}\" => {{
                        ::serde_json::from_value::<{variant_type}>(value.clone()).map(|obj| {{
                            {variant_name}(obj)
                        }}).map_err(|e| D::Error::custom(&format!(\"{{}}\", e)))
                    }}",
                    type_name = v.name.to_snake_case(),
                    variant_type = v.inner.to_rs_type(),
                    variant_name = v.qualified_name
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            subobjs = subobjs,
            variant_field = variant_field,
            on_missing_field = on_missing_field
        )
    }
}

fn obj_recur(prop: &PropType) -> Vec<String> {
    match prop {
        &PropType::Obj(ref o) => vec![o.to_code()],
        &PropType::Arr(ref prop) => obj_recur(prop),
        &PropType::Map(ref prop) => obj_recur(prop),
        &PropType::Optional(ref prop) => obj_recur(prop),
        &PropType::Enum(ref e) => vec![e.to_code()],
        _ => vec![],
    }
}

impl JsonObject {
    pub fn to_code(&self) -> String {
        let subobjs = self.fields
            .iter()
            .flat_map(|f| obj_recur(&f.ty))
            .collect::<Vec<_>>();

        let mut fields = self.fields.clone();
        fields.sort_by_key(|f| f.name.clone());

        let fields = fields.iter()
            .map(|f| f.to_code())
            .collect::<Vec<_>>();

        format!(
            "#[derive(Clone, Debug, Deserialize)]
            pub struct {name} {{
                {fields}
            }}

            {subobjs}",
            name = self.name,
            fields = fields.join("\n"),
            subobjs = subobjs.join("\n")
        )
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApiError {
    pub name: String,
    pub description: String,
}

fn format_docs(prefix: &str, s: &str) -> String {
    s.lines().map(|l| format!("{} {}\n", prefix, l)).collect()
}