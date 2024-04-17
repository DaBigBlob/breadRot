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

    #[arg(short = 'c', long = "compile-to-file")]
    compile_to_file: Option<PathBuf>,

    #[arg(short = 'a', long = "assemble-to-file")]
    assemble_to_file: Option<PathBuf>
}

// fn assemble_for(board: String) -> Option<i32> {

// }

fn main() {
    let args = Cli::parse();
    match fs::read_to_string(&args.input) {
        Ok(d) => println!("{}", d),
        Err(e) => Error::FileNotReadable(args.input.display().to_string(), e).say()
    }
}