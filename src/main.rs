mod error;

use error::Error;
use std::{fs, path::PathBuf};

use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'i', long = "input")]
    input: PathBuf,

    #[arg(short = 'b', long = "board")]
    board: String,

    #[arg(short = 'a', long = "assemble-only")]
    assemble_only: bool,

    #[arg(short = 's', long = "simulate-only")]
    simulate_only: bool
}

// fn select_assembler -> Option<i32> {

// }

fn main() {
    let args = Cli::parse();
    match fs::read_to_string(&args.input) {
        Ok(d) => println!("{}", d),
        Err(e) => Error::FileNotReadable(args.input.display().to_string(), e).say()
    }
}