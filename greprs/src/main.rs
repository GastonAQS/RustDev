extern crate greprs;

use std::env;
use std::process;
use greprs::run;
use greprs::Config;
use std::io::prelude::*;

fn main() {
    let mut stderr = std::io::stderr();
    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Problem parsing arguments: {}",err).expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        writeln!(&mut stderr, "Application error: {}",e).expect("Could not write to stderr");
        process::exit(1);
    }

}





