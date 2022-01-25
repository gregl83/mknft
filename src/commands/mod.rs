pub mod prepare;
pub mod package;
pub mod repackage;
pub mod publish;
pub mod unpublish;
pub mod reconcile;
pub mod list;

use serde::{Deserialize, Serialize};
use inflector::cases::titlecase::to_title_case;
use image::{
    ImageBuffer,
    GenericImageView,
    DynamicImage,
    Rgba
};

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

impl PackageConfig {
    fn properties_filtered(&self, filters: Vec<&str>) -> Vec<Vec<String>> {
        let mut property_filters: Vec<Vec<String>> = vec![vec![]; self.properties.len()];
        for filter in filters {
            let filter_parts: Vec<&str> = filter.split("=").collect();
            let attribute = attribute_name_format(filter_parts[0]);
            let attribute_value = attribute_name_format(filter_parts[1]);
            let property_index = self.properties.iter().position(
                |property| property.as_str() == attribute.as_str()
            ).unwrap();
            if !property_filters[property_index].contains(&attribute_value) {
                property_filters[property_index].push(attribute_value);
            }
        }
        property_filters
    }
}

pub fn attribute_name_format(attribute_name: &str) -> String {
    to_title_case(attribute_name)
}

/// write image source to target buffer
pub fn write_image(source : &DynamicImage, target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (x, y, source_pixel) in source.pixels() {
        let mut target_pixel = target.get_pixel_mut(x, y);
        if source_pixel.0[3] > 0 {
            target_pixel.0 = source_pixel.0;
        }
    }
}