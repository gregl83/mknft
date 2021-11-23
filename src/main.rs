//! Create NFT Project
//!
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

mod commands;

use clap::{SubCommand, Arg, App};

use commands::prepare;
use commands::package;

/// Run mknft.
fn main() {
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
                .index(2)))
        .get_matches();

    match matches.subcommand() {
        ("prepare", Some(matches)) => prepare::exec(matches),
        ("package", Some(matches)) => package::exec(matches),
        _ => {}
    }
}
