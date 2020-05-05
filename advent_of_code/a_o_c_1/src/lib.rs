use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::ops::AddAssign;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<String>,
}

pub struct FuelResult {
    fuel_requirement: Mutex<usize>,
}

impl FuelResult {
    fn new() -> FuelResult {
        FuelResult {
            fuel_requirement: Mutex::new(0)
        }
    }
    pub fn update_value(&self,valor: usize) {
        let mut value = self.fuel_requirement.lock().unwrap();
        *value += valor
    }
    pub fn get_value(&self) -> usize {
        let value = self.fuel_requirement.lock().unwrap();
        *value
    }
}

// impl AddAssign for FuelResult {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             fuel_requirement: self.fuel_requirement + other,
//         }
//     }
// }


trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

fn process_line(line: &str) -> usize {
    let mass = line.parse::<usize>().unwrap();
    (mass/3)-2
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let line = receiver.lock().unwrap().recv().unwrap();
                println!("{}", line);
            }
        });
        Worker {
            id,
            thread,
        }
    }
}



impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        FuelResult::new();

        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute(&self, f: String){
        let job = String::from(f);
        self.sender.send(job).unwrap();
    }
}

