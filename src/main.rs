
// Copyright (C) 2017 Brian C. Lane
//
// uses-cf is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// uses-cf is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with uses-cf.  If not, see <http://www.gnu.org/licenses/>.

extern crate clap;
extern crate reqwest;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use clap::{Arg, App};
use reqwest::Error;

fn check_domain(domain: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new().unwrap();
    let resp = client.head(&format!("https://{}",domain)).send()?;
    for header in resp.headers().iter() {
//        println!("{}: {}", header.name(), header.value_string());

        if header.name() == "cf-ray" ||
            header.value_string().contains("cloudflare") {
            return Ok(true);
        }
    }
    Ok(false)
}

fn main() {
    let matches = App::new("uses-cf")
                            .about("Check a list of websites to see who uses CloudFlare")
                            .arg(Arg::with_name("LIST")
                                        .help("List of domains to check")
                                        .required(true)
                                        .index(1))
                        .get_matches();

    let f = File::open(matches.value_of("LIST").unwrap().to_string())
                 .expect("Error opening logfile for writing.");
    let reader = BufReader::new(f);
    for l in reader.lines() {
        match l {
            Ok(domain) => {
                if domain.len() > 0 {
                    match check_domain(&domain) {
                        Ok(true) => println!("[!!] - {} uses CloudFlare, change your password.", domain),
                        Ok(false) => println!("[OK] - {} does not use CloudFlare", domain),
                        Err(e) => {
                            println!("[  ] - Error checking {}: {}", domain, e);
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }
}
