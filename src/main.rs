extern crate rand;

use rand::random;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

trait Worker {
    fn execute(&self);
}

struct Produser {
    sender: Sender<u8>,
    duration: Duration,
    count: u64,
}

impl Produser {
    fn new(sender: Sender<u8>, duration: Duration, count: u64) -> Produser {
        Produser {
            sender: sender,
            duration: duration,
            count: count,
        }
    }
}

impl Worker for Produser {
    fn execute(&self) {
        loop {
            let id = thread::current().id();
            for _ in 0..self.count {
                self.sender.send(random()).unwrap();
            }
            println!("Produse! {:?}", id);
            thread::sleep(self.duration);
        }
    }
}

struct Consumer {
    receiver: Arc<Mutex<Receiver<u8>>>,
}

impl Consumer {
    fn new(receiver: Arc<Mutex<Receiver<u8>>>) -> Consumer {
        Consumer { receiver: receiver }
    }
}

impl Worker for Consumer {
    fn execute(&self) {
        loop {
            println!(
                "recv {:?}",
                (*self.receiver.lock().unwrap()).recv().unwrap()
            );
        }
    }
}

fn main() {
    let (sender, receiver) = channel();

    let mut produsers: Vec<Produser> = vec![];
    for i in 0..2 {
        let sender = sender.clone();
        produsers.push(Produser::new(sender, Duration::from_secs(i * 3 + 1), 5 - i));
    }

    let mut prod_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for prod in produsers {
        prod_handlers.push(thread::spawn(move || {
            prod.execute();
        }));
    }

    let receiver = Arc::new(Mutex::new(receiver));
    let mut consumers: Vec<Consumer> = vec![];
    for _ in 0..2 {
        consumers.push(Consumer::new(receiver.clone()));
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
