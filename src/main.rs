mod args;
mod error;
mod logger;
mod range;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use args::Args;
use clap::Parser;
use log::info;

use crate::error::{Error, Result};
use crate::logger::init_logger;

fn main() {
    let args = Args::parse();
    init_logger(args.verbose > 0);

    info!("Input file is {:?}", &args.input_file);
    info!("Field separator is {:?}", &args.field_separation);

    let separation = match &args.field_separation {
        Some(s) => s,
        None => " ",
    };

    // let input = match &args.input_file {
    //     Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
    //     None => Box::new(std::io::stdin()) as Box<dyn Read>,
    // };
    let input = match get_input(&args.input_file) {
        Ok(i) => i,
        Err(_) => {
            log::error!("Can't open file");
            std::process::exit(1);
        }
    };

    info!("SLICE is {:?}", &args.slice);
    let ranges = match range::parse(args.slice) {
        Ok(r) => r,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    };
    info!("Parsed range is {:?}", ranges);

    let buf = BufReader::new(input);

    for line in buf.lines() {
        //println!("{}", line.unwrap());
        let a = line.unwrap();
        let syntaxs: Vec<&str> = a.split(&separation).collect();
        info!("Line: {:?}, Length: {}", &syntaxs, syntaxs.len());

        for range in &ranges {
            let start = match range.start {
                range::Index::Head(i) => i,
                range::Index::Tail(i) => syntaxs.len() - i,
            };

            let end = match range.end {
                range::Index::Head(i) => i,
                range::Index::Tail(i) => syntaxs.len() - i,
            };

            if range.is_reversed() {
                for i in (start..end).step_by(range.step).rev() {
                    print!("{} ", syntaxs[i]);
                }
            } else {
                for i in (start..end).step_by(range.step) {
                    print!("{} ", syntaxs[i]);
                }
            }
        }
        println!();
    }
}

fn get_input(file: &Option<PathBuf>) -> Result<Box<dyn Read>> {
    match &file {
        Some(f) => File::open(f)
            .map(|file| Box::new(file) as Box<dyn Read>)
            .map_err(Error::IO),
        None => Ok(Box::new(std::io::stdin()) as Box<dyn Read>),
    }
}
