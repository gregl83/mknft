use std::io::Read;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs;
use std::ops::Div;
use regex::Regex;
use clap::ArgMatches;
use psd::Psd;
use image::ImageBuffer;
use image::Rgba;
use serde_json;

use crate::commands::AttributeValue;
use crate::commands::Attribute;
use crate::commands::ProjectConfig;

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let name = matches.value_of("name").unwrap();

    let path = PathBuf::from(src);
    let dir = path.parent().unwrap();
    let config_file = fs::File::open(format!("{}/config.json", dir.as_os_str().to_str().unwrap()));

    let mut project_config = match config_file {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        _ => ProjectConfig {
            id: String::from(name),
            name: String::from(name),
            uri: None,
            attributes: vec![]
        }
    };

    let file = fs::File::open(src).expect("file should open read only");

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    let psd = Psd::from_bytes(buffer.as_slice()).unwrap();

    fs::create_dir(dest).unwrap();

    for group in psd.groups().iter() {
        let group_id = group.id() as usize;

        if let Some(layers) = psd.get_sub_layers(group_id - 1) {
            let group_name = psd.group_by_id(group_id - 1).unwrap().name();
            let skip_group = Regex::new(r"^_.*").unwrap();
            if skip_group.is_match(group_name) {
                continue;
            }
            let group_path = format!("{}/{}", dest, group_name);

            if !project_config.attributes.iter().any(|attribute| attribute.name == group_name) {
                project_config.attributes.push(Attribute {
                    name: String::from(group_name),
                    values: Vec::new()
                });
            }

            fs::create_dir(group_path).unwrap();

            for layer in layers.iter() {
                let name = layer.name();
                let skip_file = Regex::new(r"^_.*").unwrap();
                if !skip_file.is_match(name) {
                    let subpixels: Vec<u8> = layer.rgba();
                    let mut pixels = subpixels.chunks(4);

                    let mut image = ImageBuffer::new(psd.width(), psd.height());
                    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
                        let pixel_slice = pixels.next().unwrap();
                        *pixel = Rgba([
                            pixel_slice[0],
                            pixel_slice[1],
                            pixel_slice[2],
                            pixel_slice[3]
                        ]);
                    }
                    image.save(format!(
                        "{}/{}/{}.png",
                        dest,
                        group_name,
                        name
                    )).unwrap();

                    let image_path = format!("{}/{}.png", group_name, name);
                    if let Some(attribute) = project_config.attributes.iter_mut().find(|attribute| attribute.name == group_name) {
                        if let Some(attribute_value) = attribute.values.iter_mut().find(|attribute_value| attribute_value.name == name) {
                            attribute_value.path = Some(image_path);
                        } else {
                            attribute.values.push(AttributeValue {
                                name: String::from(name),
                                probability: Some(0 as f32), // set temp probability
                                path: Some(image_path),
                                weight: 10,
                                excludes: vec![]
                            });
                        }
                    }
                }
            }
        }
    }

    // calculate probabilities
    for attribute in project_config.attributes.iter_mut() {
        let attribute_weights: Vec<f32> = attribute.values.iter().map(|layer| layer.weight as f32).collect();
        let attribute_weight: f32 = attribute_weights.iter().sum();
        for attribute_value in attribute.values.iter_mut() {
            let denominator = attribute_value.weight as f32;
            attribute_value.probability = Some(denominator.div(attribute_weight));
        }
    }

    let project_config_file = fs::File::create(format!("{}/config.json", dest)).unwrap();
    serde_json::to_writer(project_config_file, &project_config).unwrap();
}