use std::collections::HashMap;
use std::path::Path;

use inflector::Inflector;

#[derive(Deserialize, Clone, Debug)]
pub struct JsonSchema {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "$schema")]
    pub schema_ref: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub properties: Option<HashMap<String, JsonSchema>>,
    pub required: Option<Vec<String>>,
    pub definitions: Option<HashMap<String, JsonSchema>>,
    pub items: Option<Box<JsonSchema>>,
    #[serde(rename = "patternProperties")]
    pub pattern_properties: Option<HashMap<String, JsonSchema>>,
    #[serde(default, rename = "additionalProperties")]
    pub additional_properties: bool,
    #[serde(rename = "$ref")]
    pub definition_ref: Option<String>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<JsonSchema>>,
}

#[derive(Clone, Debug)]
pub struct JsonObject {
    pub name: String,
    pub fields: Vec<JsonObjectFieldInfo>,
}

#[derive(Clone, Debug)]
pub struct JsonObjectFieldInfo {
    pub name: String,
    pub ty: PropType,
    pub rename: Option<String>,
}

#[derive(Clone, Debug)]
pub struct JsonEnum {
    pub name: String,
    pub variants: Vec<JsonEnumVariant>,
    pub untagged: bool,
}

#[derive(Clone, Debug)]
pub struct JsonEnumVariant {
    pub name: String,
    pub qualified_name: String,
    // TODO: this should be able to support non-containing variants
    pub inner: PropType,
}

#[derive(Clone, Debug)]
pub enum PropType {
    Str,
    Int,
    Num,
    Bool,
    Ref(String),
    Obj(JsonObject),
    Arr(Box<PropType>),
    Map(Box<PropType>),
    Optional(Box<PropType>),
    Enum(JsonEnum),
    Null,
}

impl PropType {
    pub fn from_schema(schema: &JsonSchema, name: &str) -> Self {
        if let Some(ref def) = schema.definition_ref {
            // TODO: This ignores `#/` and assumes filenames refer to an existing struct with that
            //       name.
            return PropType::Ref(Path::new(def)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                .to_pascal_case());
        }

        if let Some(ref one_of) = schema.one_of {
            return PropType::Enum(JsonEnum {
                name: name.to_owned(),
                untagged: one_of.iter().any(|o| o.ty != one_of[0].ty),
                variants: one_of.iter()
                    .map(|o| {
                        // TODO: Have this just check title. id is not reliable
                        let variant_name =
                            o.title.as_ref().or(o.id.as_ref()).unwrap().to_pascal_case();
                        let obj_name = name.to_owned() + &variant_name;
                        JsonEnumVariant {
                            name: variant_name.clone(),
                            qualified_name: format!("{}::{}", name.to_owned(), variant_name),
                            inner: Self::from_schema(o, &obj_name),
                        }
                    })
                    .collect(),
            });
        }

        match schema.ty.as_ref().map(String::as_ref) {
            Some("boolean") => PropType::Bool,
            Some("string") => PropType::Str,
            Some("integer") => PropType::Int,
            Some("number") => PropType::Num,
            Some("null") => PropType::Null,
            Some("array") => {
                // HACK: to_singular is broken in current Inflector release
                let item_name = if name.ends_with("ies") {
                    name[..name.len() - 3].to_owned() + "y"
                } else {
                    name.to_singular()
                };
                let item_schema = schema.items
                    .as_ref()
                    .expect(&format!("{} is an array but no schema is set for items", item_name));
                let subobj = Self::from_schema(&item_schema.clone(), &item_name);
                PropType::Arr(Box::new(subobj))
            }
            Some("object") => {
                if let Some(ref pp) = schema.pattern_properties {
                    let subobj_schema = pp.iter().next().unwrap().1;
                    let subobj = Self::from_schema(&subobj_schema, &name);
                    PropType::Map(Box::new(subobj))
                } else {
                    PropType::Obj(schema.properties
                        .clone()
                        .map(|p| {
                            if p.is_empty() {
                                println!("{} is an object but has no properties. Likely an error.",
                                         &name);
                            }
                            let fields = p.iter()
                                .map(|(orig_name, p)| {
                                    let (field_name, rename) = match &orig_name[..] {
                                        "type" => ("ty", Some("type".into())),
                                        "self" => ("slf", Some("self".into())),
                                        name => (name, None),
                                    };
                                    let field_ty_name = name.to_owned() +
                                                        &orig_name.to_pascal_case();
                                    let mut ty = Self::from_schema(&p, &field_ty_name);
                                    if let Some(ref req) = schema.required {
                                        if !req.contains(orig_name) {
                                            ty = PropType::Optional(Box::new(ty));
                                        }
                                    } else {
                                        ty = PropType::Optional(Box::new(ty));
                                    }
                                    JsonObjectFieldInfo {
                                        name: field_name.into(),
                                        ty: ty,
                                        rename: rename,
                                    }
                                })
                                .collect();

                            JsonObject {
                                name: name.to_owned(),
                                fields: fields,
                            }
                        })
                        .expect("Failed to get sub object for object"))
                }
            }
            ty => panic!("Unknown JSON type: {:?} for {}", ty, name),
        }
    }

    pub fn to_rs_type(&self) -> String {
        match *self {
            PropType::Str => "String".into(),
            PropType::Int => "i32".into(),
            PropType::Num => "f32".into(),
            PropType::Bool => "bool".into(),
            PropType::Null => "()".into(),
            PropType::Obj(ref obj) => obj.name.clone(),
            PropType::Ref(ref name) => format!("::{}", name),
            PropType::Arr(ref prop) => format!("Vec<{}>", prop.to_rs_type()),
            PropType::Map(ref prop) => format!("HashMap<String, {}>", prop.to_rs_type()),
            PropType::Optional(ref prop) => format!("Option<{}>", prop.to_rs_type()),
            PropType::Enum(ref e) => e.name.clone(),
        }
    }
}