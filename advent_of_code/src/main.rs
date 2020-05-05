
extern crate a_o_c_1;
use std::fs::File;
use std::io::{self, prelude::*,BufReader};
use std::path::Path;
use a_o_c_1::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let mut result: usize = 0;
    if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
        if let Ok(ip) = line {
            pool.execute(ip)
        }
        
    }
    loop {
        
    }
}

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}  