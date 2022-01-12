use std::fs;
use std::collections::HashSet;
use std::ops::Mul;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::distributions::uniform::SampleBorrow;
use clap::ArgMatches;
use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage};
use image::ImageBuffer;
use image::Rgba;
use serde_json;

use crate::commands::{
    Image,
    ProjectConfig,
    PackageConfig,
    attribute_name_format,
    write_image
};

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let size = matches.value_of("size").unwrap().parse::<u32>().unwrap();
    let order = matches.value_of("order").unwrap();

    let image_dest = format!("{}/images", dest);
    let image_temp_dest = format!("{}/tmp", dest);

    let file = fs::File::open(format!("{}/config.json", src)).expect("file should open read only");
    let project_config: ProjectConfig = serde_json::from_reader(file).unwrap();

    let mut package_config = PackageConfig {
        id: project_config.id,
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
        package_config.properties.push(
            attribute_name_format(attribute.name.as_str())
        );
    }

    let mut sampled = 0;
    let mut hashes: HashSet<String> = HashSet::new();
    while size > sampled {
        let image_name = format!("{}", sampled + 1);
        let image_uri = match project_config.uri.clone() {
            Some(project_uri) => Some(format!("{}/{}.png", project_uri, image_name.clone())),
            _ => None
        };

        // fixme - determine name of layer
        let mut package_image = Image {
            name: image_name.clone(),
            uri: image_uri,
            probability: 1.0 as f32,
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
            write_image(&image, &mut target);
            package_image.properties.push(
                attribute_name_format(image_config.name.as_str())
            );
            package_image.probability = package_image.probability.mul(image_config.probability.unwrap());
        }

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
                package_image.path = format!("images/{}.png", sampled);
                package_config.images.push(package_image);
            }
        }
    }

    if order == "probability" {
        package_config.images.sort_by(|a, b| a.probability.partial_cmp(b.probability.borrow()).unwrap());
        fs::create_dir(image_temp_dest.clone()).unwrap();
        for (index, image) in package_config.images.iter_mut().enumerate() {
            let image_name = String::from(format!("{}", index + 1)); // fixme - if naming becomes a thing
            let image_path = format!("{}/{}.png", image_temp_dest, image_name.clone());
            fs::rename(image.path.clone(), image_path.clone()).unwrap();
            image.name = image_name.clone();
            image.uri = match project_config.uri.clone() {
                Some(project_uri) => Some(format!("{}/{}.png", project_uri, image_name.clone())),
                _ => None
            };
            image.path = format!("{}/{}.png", image_dest, image_name.clone());
        }
        fs::remove_dir(image_dest.clone()).unwrap();
        fs::rename(image_temp_dest.clone(), image_dest.clone()).unwrap();
    }

    // todo - graph distribution and display

    let package_config_file = fs::File::create(format!("{}/config.json", dest)).unwrap();
    serde_json::to_writer(package_config_file, &package_config).unwrap();
}