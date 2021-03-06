extern crate argparse;

use argparse::{ArgumentParser};

fn buildCommandLine(){
    let mut source: String;
    let mut ap = ArgumentParser::new();
    ap.set_description("Synchronize the movies");
    ap.parse_args_or_exit();
}

fn main() {
    println!("Hello, world!");
    buildCommandLine()
}
