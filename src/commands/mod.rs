pub mod prepare;
pub mod package;
pub mod publish;
pub mod unpublish;
pub mod reconcile;

use serde::{Deserialize, Serialize};

/// Config exclude Attribute or AttributeValue by name
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exclude {
    pub attribute_name: String,
    pub values: Vec<String>
}

/// Project attribute value representing PSD layers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeValue {
    pub name: String,
    pub probability: Option<f32>,
    pub path: Option<String>,
    pub weight: u32,
    pub excludes: Vec<Exclude>
}

/// Project attribute having collection of values or PSD layers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub values: Vec<AttributeValue>
}

/// Project config used by `prepare` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub id: String,
    pub name: String,
    pub uri: Option<String>,
    pub attributes: Vec<Attribute>
}

/// Image (NFT) created using `package` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub name: String,
    pub uri: Option<String>,
    pub probability: f32,
    pub properties: Vec<String>,
    pub path: String,
}

/// Package config created using `package` command
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageConfig {
    pub id: String,
    pub name: String,
    pub properties: Vec<String>,
    pub images: Vec<Image>
}