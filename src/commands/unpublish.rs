use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::{ArgMatches};
use thirtyfour::GenericWebDriver;
use thirtyfour::http::reqwest_async::ReqwestDriverAsync;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::commands::PackageConfig;
use crate::adapters::metamask;

pub async fn exec(matches: &ArgMatches<'_>) {
    let src = matches.value_of("src").unwrap();
    let selenium_host = matches.value_of("host").unwrap();
    let metamask_crx = matches.value_of("crx").unwrap();
    let metamask_phrase = matches.value_of("phrase").unwrap();

    let start_arg = matches.value_of("start").unwrap();
    let start = start_arg.parse::<usize>().unwrap();
    let end_arg = matches.value_of("end").unwrap();
    let end = end_arg.parse::<usize>().unwrap();
    let wait_arg = matches.value_of("wait").unwrap();
    let wait = wait_arg.parse::<u64>().unwrap();

    let metamask_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let file = fs::File::open(format!("{}/config.json", src)).expect("file should open read only");
    let package_config: PackageConfig = serde_json::from_reader(file).unwrap();

    let mut caps = DesiredCapabilities::chrome();

    caps.add_extension(metamask_crx.as_ref()).unwrap();

    let driver = WebDriver::new(
        format!("http://{}", selenium_host).as_str(),
        &caps
    ).await.unwrap();

    // set implicit to 10 seconds; default is 0
    driver.set_implicit_wait_timeout(Duration::new(60, 0)).await.unwrap();

    // todo loop configuration looking for for mismatch:
    // - config has, site missing
    // - config has, site has duplicate
    // - site has, config missing (solve?)

    if let Err(e) = metamask::install_extension(
        &driver,
        metamask_phrase,
        metamask_password.as_str()
    ).await {
        println!("MetaMask installation error! {:?}", e);
    } else {
        match metamask::unpublish(&driver, package_config, start, end, wait).await {
            Ok(_) => println!("done!"),
            Err(e) => println!("Publish error! {:?}", e),
        }
    }

    // cleanup driver connection to selenium chrome
    driver.quit().await.unwrap();
}