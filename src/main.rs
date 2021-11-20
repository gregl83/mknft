//! Create NFT Project
//!
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use clap::{SubCommand, Arg, App};
use psd::{ColorMode, Psd, PsdChannelCompression};
use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage, Pixel};
use image::ImageBuffer;
use image::Rgba;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct AttributeValue {
    name: String,
    path: String,
    weight: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    name: String,
    values: Vec<AttributeValue>
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    weight_threshold: u32,
    attributes: Vec<Attribute>
}


/// Run mknft.
fn main() {
    // bootstrap clap cli
    let matches = App::new("mknft")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("prepare")
            .about("Prepare project using Photoshop Document")
            .arg(Arg::with_name("src")
                .help("Project psd filepath")
                .required(true)
                .index(1))
            .arg(Arg::with_name("dest")
                .help("Project output directory")
                .required(true)
                .index(2))
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .default_value("collection")
                .help("Name of NFT collection")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("package")
            .about("Package NFT project")
            .arg(Arg::with_name("src")
                .help("Project directory")
                .required(true)
                .index(1))
            .arg(Arg::with_name("dest")
                .help("Package directory")
                .required(true)
                .index(2)))
        .get_matches();

    match matches.subcommand() {
        ("prepare", Some(matches)) => {
            let name = matches.value_of("name").unwrap();
            let src = matches.value_of("src").unwrap();
            let dest = matches.value_of("dest").unwrap();

            let mut config = Config {
                name: String::from(name),
                weight_threshold: 100,
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
        ("package", Some(matches)) => {
            let src = matches.value_of("src").unwrap();
            let dest = matches.value_of("dest").unwrap();

            let file = fs::File::open(format!("{}/config.json", src)).expect("file should open read only");
            let config: Config = serde_json::from_reader(file).unwrap();

            fs::create_dir(dest).unwrap();

            // function to combine two layers of psd
            fn combine_layers(source : &DynamicImage, target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
                for (x, y, source_pixel) in source.pixels() {
                    let mut target_pixel = target.get_pixel_mut(x, y);
                    if source_pixel.0[3] > 0 {
                        target_pixel.0 = source_pixel.0;
                    }
                }
            }

            // todo - iterate layers
        }
        _ => {}
    }
}
