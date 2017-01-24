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
            "use std::collections::HashMap;
            use std::convert::From;
            use std::error::Error;
            use std::fmt;

            use ::{{ClientError, SlackWebRequestSender}};

            {methods}",
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
        let fn_name = self.name.split('.').last().unwrap().to_snake_case();
        let type_prefix = self.name.split('.').last().unwrap().to_pascal_case();
        let request_struct_name = type_prefix.clone() + "Request";
        let response_struct_name = type_prefix.clone() + "Response";
        let error_enum_name = type_prefix.clone() + "Error";
        let response = self.response.generate(&response_struct_name, &error_enum_name);

        format!(
            "{documentation}
            pub fn {method_name}<R>(client: &R, 
                                    request: &{request_type}) 
                                    -> Result<{response_type}, {error_type}>
                   where R: SlackWebRequestSender
            {{
                let mut params: HashMap<&str, String> = HashMap::new();
                {param_insertions}
                client.send(\"{name}\", params).map_err(|err| err.into())
            }}

            {request}

            {response}
            ",
            documentation = format_docs("///", &[
                &self.description,
                "",
                &format!("Wraps {}", self.documentation_url)
            ].join("\n")),
            name = self.name,
            method_name = fn_name,
            request_type = request_struct_name,
            response_type = response_struct_name,
            error_type = error_enum_name,
            response = response,
            request = self.get_request_struct(&request_struct_name),
            param_insertions = self.params.iter().map(Param::get_insertion).collect::<Vec<String>>().join("\n"),
        )
    }

    fn get_request_struct(&self, ty_name: &str) -> String {
        format!(
            "#[derive(Clone, Debug)]
            pub struct {request_type} {{
                {request_params}
            }}",
            request_type = ty_name,
            request_params = self.params.iter().map(Param::generate).collect::<Vec<String>>().join("\n")
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
                "&{enum_name}::{variant}(ref {var_name}) => {body},",
                enum_name = enm.name,
                variant = v.name,
                var_name = var_name,
                body = f(&v)
            )
        })
        .collect()
}

fn get_obj_to_response_impl(obj: &JsonObject, error_type: &str) -> String {
    format!(
        "impl ::ToResult<{name}, {error_ty}> for {name} {{
            fn to_result(&self) -> Result<{name}, {error_ty}> {{
                if self.ok {{
                    Ok(self.clone())
                }} else {{
                    Err(self.error.as_ref()
                        .map(|s| s[..].into())
                        .unwrap_or({error_ty}::MalformedResponse))
                }}
            }}
        }}",
        error_ty = error_type,
        name = obj.name
    )
}

fn get_enum_to_response_impl(enm: &JsonEnum, error_type: &str) -> String {
    format!(
        "impl ::ToResult<{name}, {error_ty}> for {name} {{
            fn to_result(&self) -> Result<{name}, {error_ty}> {{
                match self {{
                    {matches}
                }}
            }}
        }}
        
        {inner_impls}",
        error_ty = error_type,
        name = enm.name,
        matches = generate_matches(enm, "inner", |v| {
            format!("inner.to_result().clone().map(|r| {}::{}(r))", enm.name, v.name)
        }).join("\n"),
        inner_impls = enm.variants.iter()
            .map(|v| match &v.inner {
                &PropType::Obj(ref o) => get_obj_to_response_impl(o, error_type),
                &PropType::Enum(ref e) => get_enum_to_response_impl(e, error_type),
                _ => panic!("shouldn't be here...")
            })
            .collect::<Vec<_>>()
            .join("\n")
    )
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
                let to_result = if o.has_ok() { 
                    get_obj_to_response_impl(o, error_ty)
                } else {
                    "".into()
                };
                (o.to_string(), to_result)
            },
            PropType::Enum(ref e) => {
                let to_result = if e.has_ok() {
                    get_enum_to_response_impl(e, error_ty)
                } else {
                    "".into()
                };
                (e.to_string(), to_result)
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
            slack_result = to_result,
            errors = self.get_error_enum(error_ty),
        )
    }

    fn get_error_enum(&self, error_ty: &str) -> String {
        format!(
            "#[derive(Clone, Debug)]
            pub enum {error_type} {{
                {variants}
                /// The response was not \"ok\" but provided no error
                MalformedResponse,
                /// The response returned an error that was unknown to the library
                Unknown(String),
                /// The client had an error sending the request to Slack
                Client(ClientError)
            }}

            impl From<ClientError> for {error_type} {{
                fn from(err: ClientError) -> Self {{
                    {error_type}::Client(err)
                }}
            }}
            
            impl<'a> From<&'a str> for {error_type} {{
                fn from(s: &'a str) -> Self {{
                    match s {{
                        {matches}
                        _ => {error_type}::Unknown(s.to_owned())
                    }}
                }}
            }}

            impl fmt::Display for {error_type} {{
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                     write!(f, \"{{}}\", self.description())
                }}
            }}
            
            impl Error for {error_type} {{
                fn description(&self) -> &str {{
                    match self {{
                        {description_matches}
                        &{error_type}::MalformedResponse => \"Malformed response data from Slack.\",
                        &{error_type}::Unknown(ref s) => s,
                        &{error_type}::Client(ref inner) => inner.description()
                    }}
                }}

                fn cause(&self) -> Option<&Error> {{
                    match self {{
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
                        "&{error_ty}::{ty_name} => \"{str_name}\",",
                        error_ty = error_ty,
                        str_name = e.name,
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

    pub fn get_insertion(&self) -> String {
        match (&self.ty[..], self.optional) {
            ("boolean", true) => {
                format!(
                    "if let Some({name}) = request.{name} {{
                        params.insert(\"{name}\", if {name} {{
                            \"true\".to_string()
                        }} else {{
                            \"false\".to_string()
                        }});
                    }}",
                    name = self.name
                )
            }
            ("boolean", false) => {
                format!(
                    "params.insert(\"{name}\", if {name} {{
                        \"true\".to_string()
                    }} else {{
                        \"false\".to_string()
                    }});",
                    name = self.name
                )
            }
            ("integer", true) => {
                format!(
                    "if let Some({name}) = request.{name} {{
                        params.insert(\"{name}\", {name}.to_string());
                    }}",
                    name = self.name
                )
            }
            ("integer", false) => {
                format!(
                    "params.insert(\"{name}\", request.{name}.to_string());",
                    name = self.name
                )
            }
            (_, true) => {
                format!(
                    "if let Some({name}) = request.{name}.clone() {{
                        params.insert(\"{name}\", {name});
                    }}",
                    name = self.name
                )
            }
            (_, false) => {
                format!(
                    "params.insert(\"{name}\", request.{name}.clone());",
                    name = self.name
                )
            }
        }
    }

    fn get_rust_type(&self) -> String {
        let ty = match &self.ty[..] {
            "boolean" => "bool",
            "integer" => "u32",
            _ => "String",
        };
        if self.optional {
            return format!("Option<{}>", ty);
        } else {
            return ty.to_owned();
        }
    }
}

impl ToString for JsonObjectFieldInfo {
    fn to_string(&self) -> String {
        let prefix = if self.name == "error" {
            ""
        } else if self.name == "ok" {
            "#[serde(default)]"
        } else {
            "pub"
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

impl ToString for JsonEnumVariant {
    fn to_string(&self) -> String {
        format!(
            "{name}({inner}),",
            name = self.name,
            inner = self.inner.to_rs_type()
        )
    }
}

impl ToString for JsonEnum {
    fn to_string(&self) -> String {
        let subobjs = self.variants
            .iter()
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
                    if let Some(ty_val) = value.find(\"type\") {{
                        if let Some(ty) = ty_val.as_str() {{
                            match ty {{
                                {variant_matches}
                                _ => Err(D::Error::unknown_variant(ty, VARIANTS))
                            }}
                        }} else {{
                            Err(D::Error::invalid_type(::serde::de::Unexpected::Unit, &\"a string\"))
                        }}
                    }} else {{
                        Err(D::Error::missing_field(\"type\"))
                    }}
                }}
            }}

            {subobjs}",
            name = self.name,
            variants = self.variants
                .iter()
                .map(ToString::to_string)
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
                            {enum_name}::{variant_name}(obj)
                        }}).map_err(|e| D::Error::custom(&format!(\"{{}}\", e)))
                    }}",
                    type_name = match &v.inner {
                        &PropType::Obj(ref o) => {
                            // awful hack to get the JSON enum type name back out
                            let snake_name = o.name.to_snake_case();
                            snake_name.split('_').last().unwrap().to_owned()
                        },
                        _ => panic!()
                    },
                    variant_type = v.inner.to_rs_type(),
                    enum_name = self.name,
                    variant_name = v.name
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            subobjs = subobjs
        )
    }
}

fn obj_recur(prop: &PropType) -> Vec<String> {
    match prop {
        &PropType::Obj(ref o) => vec![o.to_string()],
        &PropType::Arr(ref prop) => obj_recur(prop),
        &PropType::Map(ref prop) => obj_recur(prop),
        &PropType::Optional(ref prop) => obj_recur(prop),
        &PropType::Enum(ref e) => vec![e.to_string()],
        _ => vec![],
    }
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        let subobjs = self.fields
            .iter()
            .flat_map(|f| obj_recur(&f.ty))
            .collect::<Vec<_>>();

        let mut fields = self.fields.clone();
        fields.sort_by_key(|f| f.name.clone());

        let fields = fields.iter()
            .map(ToString::to_string)
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