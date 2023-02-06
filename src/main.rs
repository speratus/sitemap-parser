use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

use std::path::PathBuf;
use std::fs;

use clap::{arg, command, value_parser};

use csv::Writer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "url")]
struct Url {
    loc: String,
    lastmod: String,
    priority: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "urlset")]
struct Urlset {
    url: Vec<Url>,
}

fn main() -> csv::Result<()> {
    
    let matches = command!()
        .arg(
            arg!(-i --input <FILE> "Sets the input file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-o --output <FILE> "Sets the output file")
            .required(true)
            .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

        let input: &PathBuf;

        if let Some(raw_input) = matches.get_one::<PathBuf>("input") {
            input = raw_input;
        } else {
            panic!("No input file!")
        }

        let output: &PathBuf;

        if let Some(raw_output) = matches.get_one::<PathBuf>("output") {
            output = raw_output;
        } else {
            panic!("No output file!")
        }


        let contents = fs::read_to_string(input).expect("Could not open file!");

        let urlset: Urlset = from_str(contents.as_str()).unwrap();

        let urls: Vec<Url> = urlset.url;


        let mut wrtr = Writer::from_path(output)?;

        for u in urls {
            wrtr.serialize(u)?;
        }

        Ok(())
}
