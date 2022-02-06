//! Create NFT Project
//!
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

mod commands;

use tokio;
use clap::{SubCommand, Arg, App, arg_enum};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Order {
        Probability,
        Random
    }
}

/// Run mknft.
#[tokio::main]
async fn main() {
    // bootstrap clap cli
    let matches = App::new("mknft")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("prepare")
            .about("Prepare NFT project using Photoshop Document")
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
                .index(2))
            .arg(Arg::with_name("size")
                .help("Size of collection (near combination limit can loop indefinitely)")
                .required(true)
                .index(3))
            .arg(Arg::with_name("order")
                .short("o")
                .long("order")
                .value_name("ORDER")
                .default_value("random")
                .help("Order of collection")
                .takes_value(true)
                .possible_values(&Order::variants())
                .case_insensitive(true)))
        .subcommand(SubCommand::with_name("repackage")
            .about("Repackage NFT package")
            .arg(Arg::with_name("src_project")
                .help("Project directory")
                .required(true)
                .index(1))
            .arg(Arg::with_name("src_package")
                .help("Package directory")
                .required(true)
                .index(2))
            .arg(Arg::with_name("dest")
                .help("Package directory")
                .required(true)
                .index(3)))
        .get_matches();

    match matches.subcommand() {
        ("prepare", Some(matches)) => commands::prepare::exec(matches).await,
        ("package", Some(matches)) => commands::package::exec(matches).await,
        ("repackage", Some(matches)) => commands::repackage::exec(matches).await,
        _ => {}
    }
}
