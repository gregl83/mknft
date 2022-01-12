use std::borrow::Borrow;
use std::fs;
use clap::ArgMatches;
use image::{
    ImageBuffer,
    io::Reader as ImageReader,
    GenericImageView,
    DynamicImage,
    Rgba
};
use serde_json;

use crate::commands::{
    ProjectConfig,
    PackageConfig,
    attribute_name_format,
    write_image
};

pub async fn exec(matches: &ArgMatches<'_>) {
    let src_project = matches.value_of("src_project").unwrap();
    let src_package = matches.value_of("src_package").unwrap();
    let dest = matches.value_of("dest").unwrap();

    let image_dest = format!("{}/images", dest);

    let file_project = fs::File::open(
        format!("{}/config.json", src_project)
    ).expect("file should open read only");
    let project_config: ProjectConfig = serde_json::from_reader(file_project).unwrap();

    let file_package = fs::File::open(
        format!("{}/config.json", src_package)
    ).expect("file should open read only");
    let mut package_config: PackageConfig = serde_json::from_reader(file_package).unwrap();

    fs::create_dir(dest).unwrap();
    fs::create_dir(image_dest.clone()).unwrap();

    // create layer image buffers
    let mut layers: Vec<Vec<(String, DynamicImage)>> = vec![];
    for attribute in project_config.attributes.iter() {
        let mut images: Vec<(String, DynamicImage)> = vec![];
        for value in attribute.values.iter() {
            if let Some(value_path) = &value.path {
                let image_buffer = ImageReader::open(
                    format!("{}/{}", src_project, value_path)
                ).unwrap().decode().unwrap();
                images.push(
                    (attribute_name_format(value.name.as_str()), image_buffer)
                );
            }
        }
        layers.push(images);
    }

    // write images
    for (image_index, image) in package_config.images.iter_mut().enumerate() {
        let layer_sample = layers[0][0].1.borrow();
        let mut target: ImageBuffer::<Rgba<u8>, Vec<_>> = ImageBuffer::new(
        layer_sample.width(),
        layer_sample.height()
        );

        for (index, attribute_value) in image.properties.iter().enumerate() {
            let (_, image) = layers[index].iter().find(|(s, _)| {
                s.as_str() == attribute_value.as_str()
            }).unwrap();
            write_image(&image, &mut target);
        }

        let image_path = format!("{}/{}.png", image_dest.clone(), image_index + 1);
        image.path = format!("images/{}.png", image_index + 1);
        target.save(image_path.clone()).unwrap();
    }

    let package_config_file = fs::File::create(
        format!("{}/config.json", dest)
    ).unwrap();
    serde_json::to_writer(package_config_file, &package_config).unwrap();
}