use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Specify field separator
    #[arg(short, long, name = "REGEX")]
    pub field_separation: Option<String>,

    /// Path to input file
    #[arg(short, long, name = "FILE", default_value = None)]
    pub input_file: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Python like slice, like 1:10:2
    #[arg()]
    slice: Vec<String>,
}
