use clap::{SubCommand, ArgMatches, Arg, App};
use thirtyfour::prelude::*;

use crate::commands::AttributeValue;
use crate::commands::Attribute;
use crate::commands::ProjectConfig;

pub async fn exec(matches: &ArgMatches<'_>) {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();

    // Navigate to https://wikipedia.org.
    driver.get("https://google.com").await.unwrap();
    let elem_form = driver.find_element(By::Id("search-form")).await.unwrap();

    // Find element from element.
    let elem_text = elem_form.find_element(By::Id("searchInput")).await.unwrap();

    // Type in the search terms.
    elem_text.send_keys("selenium").await.unwrap();

    // Click the search button.
    let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await.unwrap();
    elem_button.click().await.unwrap();

    // Look for header to implicitly wait for the page to load.
    driver.find_element(By::ClassName("firstHeading")).await.unwrap();
    assert_eq!(driver.title().await.unwrap(), "Selenium - Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await.unwrap();
}