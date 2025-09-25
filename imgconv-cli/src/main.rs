use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Input file path
    input: PathBuf,
    /// Output file path
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    match imgconv_core::convert_image(&args.input, &args.output) {
        Ok(_) => println!("Converted {:?} -> {:?}", args.input, args.output),
        Err(e) => eprintln!("Error: {}", e),
    }
}