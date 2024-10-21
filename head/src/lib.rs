#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short('n'), long("lines"), default_value = "10")]
    lines: u64,
    #[arg(short('c'), long("bytes"), conflicts_with("lines"))]
    bytes: Option<u64>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        dbg!(self);
        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
