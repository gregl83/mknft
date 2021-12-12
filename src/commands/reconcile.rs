use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::{ArgMatches};

use crate::commands::PackageConfig;

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    // todo - do something useful
}