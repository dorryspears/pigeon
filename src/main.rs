use std::process::exit;
use clap::Parser;
use structs::args::Args;
pub mod structs;

fn main() {
    let args = Args::parse();

    if !args.is_validate_request_type() {
        println!("You need to pass one of -g, -p, -d, -u");
        exit(1)
    } 
}