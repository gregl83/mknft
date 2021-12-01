//! Create NFT Project
//!
//!
//! For help:
//! ```bash
//! cargo run -- -h
//! ```

mod commands;

use tokio;
use clap::{SubCommand, Arg, App};

use commands::prepare;
use commands::package;
use commands::publish;

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
                .index(3)))
        .subcommand(SubCommand::with_name("publish")
            .about("Publish NFT package to OpenSea Collection")
            .arg(Arg::with_name("src")
                .help("Package directory")
                .required(true)
                .index(1))
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .default_value("localhost:4444")
                .help("Host to Selenium Chrome")
                .takes_value(true))
            .arg(Arg::with_name("crx")
                .short("x")
                .long("crx")
                .value_name("CRX")
                .default_value("MetaMask.crx")
                .help("MetaMask CRX path")
                .takes_value(true))
            .arg(Arg::with_name("phrase")
                .short("p")
                .long("phrase")
                .value_name("PHRASE")
                .help("MetaMask recovery phrase for wallet authentication")
                .takes_value(true)))
        .get_matches();

    match matches.subcommand() {
        ("prepare", Some(matches)) => prepare::exec(matches).await,
        ("package", Some(matches)) => package::exec(matches).await,
        ("publish", Some(matches)) => publish::exec(matches).await,
        _ => {}
    }
}
