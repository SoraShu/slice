mod args;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Field separator is {:?}", &args.field_separation);
    println!("Input file is {:?}", &args.input_file);
    println!("Verbose is {}", &args.verbose);

    let input = match &args.input_file {
        Some(f) => Box::new(File::open(f).expect("Could not open file")) as Box<dyn Read>,
        None => Box::new(std::io::stdin()) as Box<dyn Read>,
    };

    let buf = BufReader::new(input);

    for line in buf.lines() {
        println!("{}", line.unwrap());
    }

    println!("Hello, world!");
}
