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
pub struct Config {
    name: String,
    attributes: Vec<Attribute>
}