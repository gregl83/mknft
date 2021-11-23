use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs;
use regex::Regex;
use clap::{SubCommand, ArgMatches, Arg, App};
use psd::{ColorMode, Psd, PsdChannelCompression};
use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage, Pixel};
use image::ImageBuffer;
use image::Rgba;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::commands::AttributeValue;
use crate::commands::Attribute;
use crate::commands::Config;

pub fn exec(matches: &ArgMatches) {
    let name = matches.value_of("name").unwrap();
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();

    let mut config = Config {
        name: String::from(name),
        attributes: Vec::new()
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
            let group_path = format!("{}/{}", dest, group_name);

            config.attributes.push(Attribute {
                name: String::from(group_name),
                values: Vec::new()
            });

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
                    ));

                    for attribute in config.attributes.iter_mut() {
                        if attribute.name.as_str() == group_name {
                            attribute.values.push(AttributeValue {
                                name: String::from(name),
                                path: format!(
                                    "{}/{}.png",
                                    group_name,
                                    name
                                ),
                                weight: 10
                            });
                            break;
                        }
                    }
                }
            }
        }
    }

    let config_file = fs::File::create(format!("{}/config.json", dest)).unwrap();
    serde_json::to_writer(config_file, &config);

}