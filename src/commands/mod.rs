pub mod prepare;
pub mod package;
pub mod publish;

use serde::{Deserialize, Serialize};

/// Config exclude Attribute or AttributeValue by name
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exclude {
    attribute_name: String,
    values: Vec<String>
}

/// Project attribute value representing PSD layers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeValue {
    name: String,
    probability: Option<f32>,
    path: Option<String>,
    weight: u32,
    excludes: Vec<Exclude>
}

/// Project attribute having collection of values or PSD layers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    name: String,
    values: Vec<AttributeValue>
}

/// Project config used by `prepare` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    id: String,
    name: String,
    uri: Option<String>,
    attributes: Vec<Attribute>
}

/// Image (NFT) created using `package` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    name: String,
    uri: Option<String>,
    probability: f32,
    properties: Vec<String>,
    path: String,
}

/// Package config created using `package` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageConfig {
    id: String,
    name: String,
    properties: Vec<String>,
    images: Vec<Image>
}