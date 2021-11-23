pub mod prepare;
pub mod package;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeValue {
    name: String,
    path: String,
    weight: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    values: Vec<AttributeValue>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    attributes: Vec<Attribute>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    name: String,
    properties: Vec<String>,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    name: String,
    properties: Vec<String>,
    images: Vec<Image>
}