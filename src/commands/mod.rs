pub mod prepare;
pub mod package;
pub mod publish;

use serde::{Deserialize, Serialize};

/// Project attribute value representing PSD layers
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    name: String,
    path: String,
    weight: u32
}

/// Project attribute having collection of values or PSD layers
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    values: Vec<AttributeValue>
}

/// Config for `mknft` used when parsing PSD
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    attributes: Vec<Attribute>
}

/// Project config prepared using `prepare` command
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    attributes: Vec<Attribute>
}

/// Image (NFT) packaged using `package` command
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    name: String,
    properties: Vec<String>,
    path: String,
}

/// Package config packaged using `package` command
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    name: String,
    properties: Vec<String>,
    images: Vec<Image>
}