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

use crate::commands::Config;

pub fn exec(matches: &ArgMatches) {
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

    let mut rng = thread_rng();

    let mut layers: Vec<(String, Vec<DynamicImage>, WeightedIndex<u32>)> = vec![];
    for attribute in config.attributes.iter() {
        let mut index: Vec<u32> = vec![];
        let mut images: Vec<DynamicImage> = vec![];
        for value in attribute.values.iter() {
            index.push(value.weight);
            images.push(ImageReader::open(
            format!("{}/{}", src, value.path)
            ).unwrap().decode().unwrap());
        }
        layers.push((
            attribute.name.clone(),
            images,
            WeightedIndex::new(index).unwrap()
        ));
    }
    layers.reverse();

    let sample_size = 6;
    let mut sampled = 0;
    let mut image_ids = HashSet::new();

    while sample_size > sampled {
        let mut target: ImageBuffer::<Rgba<u8>, Vec<_>> = ImageBuffer::new(
        layers[0].1[0].width(),
        layers[0].1[0].height()
        );

        let mut image_id = String::new();
        for (_name, images, weights) in layers.iter() {
            let index = weights.sample(&mut rng);
            image_id = format!("{}{}", image_id, index);
            let image = images.get(index).unwrap();
            combine_layers(&image, &mut target);
        }

        if !image_ids.contains(image_id.as_str()) {
            sampled += 1;
            target.save(format!("{}/{}.png", dest, sampled)).unwrap();
            image_ids.insert(image_id.to_owned());
        }
    }

    // todo - write config

}