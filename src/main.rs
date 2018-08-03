extern crate rand;

use rand::random;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

trait Worker {
    fn execute(&self);
}

struct Produser {
    duration: Duration,
    count: u64,
    deque: Arc<Mutex<VecDeque<u8>>>,
}

impl Produser {
    fn new(duration: Duration, count: u64, mutex: Arc<Mutex<VecDeque<u8>>>) -> Produser {
        Produser {
            duration: duration,
            count: count,
            deque: mutex,
        }
    }
}

impl Worker for Produser {
    fn execute(&self) {
        loop {
            let mut deque = self.deque.lock().unwrap();
            let id = thread::current().id();
            print!("Produse {:?}: ", id);
            for _ in 0..self.count {
                let value = random();
                print!("{} ", value);
                (*deque).push_back(value);
            }
            println!("");
            thread::sleep(self.duration);
        }
    }
}

struct Consumer {
    deque: Arc<Mutex<VecDeque<u8>>>,
}

impl Consumer {
    fn new(deque: Arc<Mutex<VecDeque<u8>>>) -> Consumer {
        Consumer { deque: deque }
    }
}

impl Worker for Consumer {
    fn execute(&self) {
        loop {
            println!(
                "recv {:?}",
                (*self.deque.lock().unwrap()).pop_front().unwrap()
            );
        }
    }
}

fn main() {
    let deque: Arc<Mutex<VecDeque<u8>>> = Arc::new(Mutex::new(VecDeque::new()));

    let mut produsers: Vec<Produser> = vec![];
    for i in 0..2 {
        produsers.push(Produser::new(
            Duration::from_secs(i * 3 + 1),
            5 - i,
            deque.clone(),
        ));
    }

    let mut prod_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for prod in produsers {
        prod_handlers.push(thread::spawn(move || {
            prod.execute();
        }));
    }

    let mut consumers: Vec<Consumer> = vec![];
    for _ in 0..2 {
        consumers.push(Consumer::new(deque.clone()));
    }

    let mut cons_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for cons in consumers {
        cons_handlers.push(thread::spawn(move || {
            cons.execute();
        }));
    }

    for handler in prod_handlers {
        handler.join().unwrap();
    }

    for handler in cons_handlers {
        handler.join().unwrap();
    }
}
