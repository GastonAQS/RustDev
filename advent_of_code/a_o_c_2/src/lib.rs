extern crate threadpool;
extern crate num_cpus;
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use threadpool::ThreadPool;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;


pub struct MyThreadpool {
    threads: ThreadPool,
    result: Arc<AtomicUsize>,

}

impl MyThreadpool {
    pub fn new() -> MyThreadpool {
        let pool = ThreadPool::with_name("my_threadpool".to_owned(), num_cpus::get());
        let result = Arc::new(AtomicUsize::new(0));
        MyThreadpool {
            threads: pool,
            result
        }
    }

    pub fn execute(&self) {
        let (tx,rx) = mpsc::channel();
        if let Ok(lines) = read_file("input.txt") {
            for line in lines {
                let t_result = self.result.clone();
                if let Ok(ip) = line {
                    println!("{}",ip);
                    self.threads.execute(move || {
                        t_result.fetch_add(calculate_mass(&ip),Ordering::Relaxed);
                    });
                };
                
            };
        };
        self.threads.join();
        let final_result = Arc::try_unwrap(self.result.clone()).unwrap().into_inner();
        println!("Final result is: {}",final_result);
        // Shows the correct value but the threads panic and does not print it in console
    }
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}  

fn calculate_mass(value: &str) -> usize {
    let val = value.parse::<usize>().unwrap();
    (val/3)-2
}
