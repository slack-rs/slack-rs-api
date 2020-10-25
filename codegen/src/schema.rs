use crate::vec_or_single::VecOrSingle;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    #[serde(rename = "basePath")]
    pub base_path: String,
    pub definitions: BTreeMap<String, Schema>,
    pub external_docs: ExternalDoc,
    pub host: String,
    pub info: Info,
    pub paths: BTreeMap<String, PathItem>,
    pub schemes: Vec<Scheme>,
    pub security_definitions: BTreeMap<String, Security>,
    pub swagger: String,
    pub tags: Vec<Tag>,
}

impl Spec {
    pub fn replace_refs(&mut self) -> Result<()> {
        for path in self.paths.values_mut() {
            for mut operation in &mut [&mut path.get, &mut path.post] {
                if let Some(ref mut op) = &mut operation {
                    for response in op.responses.values_mut() {
                        Self::replace_refs_schema(&mut response.schema, &self.definitions)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn replace_refs_schema(
        schema: &mut Schema,
        definitions: &BTreeMap<String, Schema>,
    ) -> Result<()> {
        if let Some(ref_path) = &schema.ref_path {
            let def_name = ref_path.trim_start_matches("#/definitions/");
            let def = definitions
                .get(def_name)
                .with_context(|| format!("Definition for {} is missing", def_name))?;
            *schema = def.clone();
        }
        if let Some(items) = &mut schema.items {
            for item in &mut items.0 {
                Self::replace_refs_schema(item, definitions)?;
            }
        }
        if let Some(properties) = &mut schema.properties {
            for property in properties.values_mut() {
                Self::replace_refs_schema(property, definitions)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<VecOrSingle<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<EnumValues>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "minItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub schema_type: Option<VecOrSingle<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x-examples")]
    pub examples: Option<VecOrSingle<String>>,
    #[serde(rename = "uniqueItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_path: Option<String>,
}

impl Schema {
    pub fn merge(&mut self, other: &Schema) {
        match (&mut self.items, &other.items) {
            (_, None) => {}
            (None, o) => self.items = o.clone(),
            (Some(ref mut s), Some(ref o)) => s.extend_from_slice(&o),
        }
        match (&mut self.enum_values, &other.enum_values) {
            (_, None) => {}
            (None, o) => self.enum_values = o.clone(),
            (Some(ref mut s), Some(ref o)) => s.extend_from_slice(&o),
        }
        match (&mut self.properties, &other.properties) {
            (_, None) => {}
            (None, o) => self.properties = o.clone(),
            (Some(ref mut s), Some(ref o)) => {
                for (key, value) in o {
                    let _ = s.entry(key.to_string()).or_insert_with(|| value.clone());
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(bool),
    Type {
        #[serde(rename = "type")]
        param_type: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum EnumValues {
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    pub description: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    pub contact: Contact,
    pub description: String,
    pub title: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Operation {
    pub consumes: Vec<String>,
    pub description: String,
    #[serde(rename = "externalDocs")]
    pub external_docs: ExternalDoc,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub parameters: Vec<Parameter>,
    pub produces: Vec<String>,
    pub responses: BTreeMap<String, Response>,
    pub security: Vec<SecurityRequirement>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub location: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    pub description: String,
    pub schema: Schema,
}

pub type SecurityRequirement = BTreeMap<String, Vec<String>>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Https,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename = "oauth2")]
    Oauth2 {
        #[serde(rename = "authorizationUrl")]
        authorization_url: String,
        flow: Flow,
        scopes: BTreeMap<String, String>,
        #[serde(rename = "tokenUrl")]
        token_url: String,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Flow {
    Implicit,
    Password,
    Application,
    AccessCode,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub external_docs: Vec<ExternalDoc>,
}
