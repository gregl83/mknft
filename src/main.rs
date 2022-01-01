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

use commands::prepare;
use commands::package;
use commands::publish;
use commands::unpublish;
use commands::reconcile;

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
                .takes_value(true))
            .arg(Arg::with_name("start")
                .short("s")
                .long("start")
                .value_name("NUMBER")
                .default_value("0")
                .help("Start position of NFTs to publish (not included)")
                .takes_value(true)
                .case_insensitive(true))
            .arg(Arg::with_name("end")
                .short("e")
                .long("end")
                .value_name("NUMBER")
                .default_value("10000")
                .help("End position of NFTs to publish (included)")
                .takes_value(true)
                .case_insensitive(true))
            .arg(Arg::with_name("wait")
                    .short("w")
                    .long("wait")
                    .value_name("DURATION")
                    .default_value("20")
                    .help("Wait duration (seconds) between chunks (10) to prevent rate limit errors")
                    .takes_value(true)
                    .case_insensitive(true)))
        .subcommand(SubCommand::with_name("reconcile")
            .about("Reconcile published NFT package for anomalies with OpenSea Collection")
            .arg(Arg::with_name("src")
                .help("Package directory")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("unpublish")
            .about("Unpublish NFT package from OpenSea Collection")
            .arg(Arg::with_name("src")
                .help("Package directory")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        ("prepare", Some(matches)) => prepare::exec(matches).await,
        ("package", Some(matches)) => package::exec(matches).await,
        ("publish", Some(matches)) => publish::exec(matches).await,
        ("reconcile", Some(matches)) => publish::exec(matches).await,
        _ => {}
    }
}
