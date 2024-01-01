mod args;
mod logger;
mod range;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use args::Args;
use clap::Parser;
use log::info;

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

    let input = match &args.input_file {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
        None => Box::new(std::io::stdin()) as Box<dyn Read>,
    };

    info!("SLICE is {:?}", &args.slice);
    let ranges = range::parse(args.slice);
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
