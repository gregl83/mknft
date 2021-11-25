use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
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

use crate::commands::Image;
use crate::commands::ProjectConfig;
use crate::commands::PackageConfig;

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

    // function to combine two layers of psd
    fn combine_layers(source : &DynamicImage, target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        for (x, y, source_pixel) in source.pixels() {
            let mut target_pixel = target.get_pixel_mut(x, y);
            if source_pixel.0[3] > 0 {
                target_pixel.0 = source_pixel.0;
            }
        }
    }

    let mut rng = thread_rng();

    let mut layers: Vec<(Vec<(String, DynamicImage)>, WeightedIndex<u32>)> = vec![];
    for attribute in project_config.attributes.iter() {
        let mut index: Vec<u32> = vec![];
        let mut images: Vec<(String, DynamicImage)> = vec![];
        for value in attribute.values.iter() {
            index.push(value.weight);
            images.push((
                value.name.clone(),
                ImageReader::open(format!("{}/{}", src, value.path)).unwrap().decode().unwrap()
            ));
        }
        layers.push((
            images,
            WeightedIndex::new(index).unwrap()
        ));
        package_config.properties.push(attribute.name.clone());
    }

    let sample_size = 6;
    let mut sampled = 0;
    let mut image_ids = HashSet::new();

    while sample_size > sampled {
        let mut package_image = Image {
            name: format!("{}", sampled + 1),
            properties: vec![],
            path: String::new()
        };

        let mut target: ImageBuffer::<Rgba<u8>, Vec<_>> = ImageBuffer::new(
        layers[0].0[0].1.width(),
        layers[0].0[0].1.height()
        );

        let mut image_id = String::new();
        for (images, weights) in layers.iter() {
            let index = weights.sample(&mut rng);
            image_id = format!("{}{}", image_id, index);
            let (name, image) = images.get(index).unwrap();
            combine_layers(&image, &mut target);
            package_image.properties.push(name.clone());
        }

        if !image_ids.contains(image_id.as_str()) {
            sampled += 1;
            let image_path = format!("{}/{}.png", image_dest.clone(), sampled);
            target.save(image_path.clone()).unwrap();
            image_ids.insert(image_id.to_owned());
            package_image.path = image_path;
            package_config.images.push(package_image);
        }
    }

    let package_config_file = fs::File::create(format!("{}/config.json", dest)).unwrap();
    serde_json::to_writer(package_config_file, &package_config);
}