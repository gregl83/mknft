use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::{ArgMatches};

use crate::commands::PackageConfig;

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    // todo - do something useful

    // todo get configuration

    // todo loop configuration looking for for mismatch:
    // - config has, site missing
    // - config has, site has duplicate
    // - site has, config missing (solve?)





}