#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::{command, Arg, ArgAction};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Config {
    pub fn try_from_args() -> Result<Self> {
        let matches = command!()
            .arg(
                Arg::new("files")
                    .value_name("FILES")
                    .help("Input file(s)")
                    .required(false)
                    .default_values(vec!["-"])
                    .action(ArgAction::Append),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .help("Number lines")
                    .conflicts_with("number_nonblank")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("number_nonblank")
                    .short('b')
                    .long("number-nonblank")
                    .help("Number non blank lines")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();
        let files = matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>();
        let number_lines = matches.get_flag("number");
        let number_nonblank_lines = matches.get_flag("number_nonblank");

        Ok(Self {
            files,
            number_lines,
            number_nonblank_lines,
        })
    }

    pub fn run(&self) -> Result<()> {
        for filename in self.files.clone() {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {filename}: {err}"),
                Ok(file) => {
                    let mut line_count = 0;
                    for line_result in file.lines() {
                        let line = line_result?;
                        match (self.number_lines, self.number_nonblank_lines) {
                            (true, false) => {
                                line_count += 1;
                                println!("{line_count:6}\t{line}");
                            }
                            (false, true) => {
                                if line.is_empty() {
                                    println!("{line}");
                                } else {
                                    line_count += 1;
                                    println!("{line_count:6}\t{line}");
                                }
                            }
                            (false, false) => println!("{line}"),
                            (true, true) => unreachable!(
                                "Shouldn't be possible, both options are mutually exclusive"
                            ),
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
