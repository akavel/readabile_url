use core::time::Duration;
use std::process;

use anyhow::Result;
use readability::extractor as readability;
use reqwest::Url;

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip name of binary
    if args.len() != 1 {
        eprintln!(
            "readabile_url expects exactly 1 argument, an URL; got {} args",
            args.len()
        );
        process::exit(1);
    }
    let url = args.next().unwrap();
    match scrape(&url) {
        Ok(product) => {
            println!("{}", product.content);
        }
        Err(err) => {
            eprintln!("readabile_url error: {}", err);
            process::exit(1);
        }
    }
}

pub fn scrape(url: &str) -> Result<readability::Product> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let mut res = client.get(url).send()?.error_for_status()?;
    let url = Url::parse(url)?;
    Ok(readability::extract(&mut res, &url)?)
}
