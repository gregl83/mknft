use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::format;
use std::hash::Hash;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use regex::Regex;
use clap::{SubCommand, Arg, ArgMatches, App};
use psd::{ColorMode, Psd, PsdChannelCompression};
use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage, Pixel};
use image::ImageBuffer;
use image::Rgba;
use serde::{Deserialize, Serialize};
use serde_json;
use inflector::cases::titlecase::to_title_case;

use crate::commands::Image;
use crate::commands::ProjectConfig;
use crate::commands::PackageConfig;

// function to combine two layers of psd
fn combine_layers(source : &DynamicImage, target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (x, y, source_pixel) in source.pixels() {
        let mut target_pixel = target.get_pixel_mut(x, y);
        if source_pixel.0[3] > 0 {
            target_pixel.0 = source_pixel.0;
        }
    }
}

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let image_dest = format!("{}/images", dest);

    let file = fs::File::open(format!("{}/config.json", src)).expect("file should open read only");
    let project_config: ProjectConfig = serde_json::from_reader(file).unwrap();

    let mut package_config = PackageConfig {
        name: project_config.name,
        properties: vec![],
        images: vec![]
    };

    fs::create_dir(dest).unwrap();
    fs::create_dir(image_dest.clone()).unwrap();

    let mut rng = thread_rng();

    // create layer image buffers and weighted index for random selection
    let mut layers: Vec<(Vec<DynamicImage>, WeightedIndex<u32>)> = vec![];
    for attribute in project_config.attributes.iter() {
        let mut index: Vec<u32> = vec![];
        let mut images: Vec<DynamicImage> = vec![];
        for value in attribute.values.iter() {
            if let Some(value_path) = &value.path {
                index.push(value.weight);
                images.push(
                    ImageReader::open(format!("{}/{}", src, value_path)).unwrap().decode().unwrap()
                );
            }
        }
        layers.push((
            images,
            WeightedIndex::new(index).unwrap()
        ));
        package_config.properties.push(attribute.name.clone());
    }

    let sample_size = 6;
    let mut sampled = 0;
    let mut hashes: HashSet<String> = HashSet::new();

    while sample_size > sampled {
        // fixme - determine name of layer
        let mut package_image = Image {
            name: format!("{}", sampled + 1),
            properties: vec![],
            path: String::new()
        };

        let mut target: ImageBuffer::<Rgba<u8>, Vec<_>> = ImageBuffer::new(
        layers[0].0[0].width(),
        layers[0].0[0].height()
        );

        // set excludes tracking variables
        let mut used: Vec<String> = vec![];
        let mut excluded: HashSet<String> = HashSet::new();

        // generate image from random weighted selection of layers
        for (index, attribute) in project_config.attributes.iter().enumerate() {
            let (images, weighted_index) = layers.get(index).unwrap();
            let image_index = weighted_index.sample(&mut rng);
            let image_config = attribute.values.get(image_index).unwrap();
            for exclude in image_config.excludes.iter() {
                for exclude_image in exclude.values.iter() {
                    excluded.insert(format!("{}:{}", exclude.attribute_name.clone(), exclude_image.clone()));
                }
            }
            used.push(format!("{}:{}", attribute.name.clone(), image_config.name.clone()));
            let image = images.get(image_index).unwrap();
            combine_layers(&image, &mut target);
            package_image.properties.push(
                to_title_case(image_config.name.as_str())
            );
        }

        // todo - calculate rarity of each item
        // todo - re-order by rarity
        // todo - blank image for each layer

        // check for exclusion collision
        let mut exclude_collision = false;
        for u in used {
            if excluded.contains(u.as_str()) {
                exclude_collision = true;
                break;
            }
        }
        if !exclude_collision {
            // generate image hash for duplicate detection
            let mut hasher = Sha256::new();
            hasher.update(target.as_ref());
            let hasher_result = hasher.finalize();
            let hash = format!("{:X}", hasher_result);

            // generate image if new/unique
            if !hashes.contains(hash.as_str()) {
                sampled += 1;
                let image_path = format!("{}/{}.png", image_dest.clone(), sampled);
                target.save(image_path.clone()).unwrap();
                hashes.insert(hash.to_owned());
                package_image.path = image_path;
                package_config.images.push(package_image);
            }
        }
    }

    let package_config_file = fs::File::create(format!("{}/config.json", dest)).unwrap();
    serde_json::to_writer(package_config_file, &package_config);
}