use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

trait Worker {
    fn execute(&self);
}

struct Produser {
    sender: mpsc::Sender<thread::ThreadId>,
    duration: Duration,
    count: u64,
}

impl Produser {
    fn new(sender: mpsc::Sender<thread::ThreadId>, duration: Duration, count: u64) -> Produser {
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
                self.sender.send(id).unwrap();
            }
            println!("Produse! {:?}", id);
            thread::sleep(self.duration);
        }
    }
}

struct Consumer {
    receiver: Arc<Mutex<mpsc::Receiver<thread::ThreadId>>>,
}

impl Consumer {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<thread::ThreadId>>>) -> Consumer {
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
    let (sender, receiver) = mpsc::channel();

    let mut produsers: Vec<Produser> = vec![];
    for i in 0..5 {
        let sender = sender.clone();
        produsers.push(Produser::new(sender, Duration::from_secs(i + 1), 5 - i));
    }

    let mut prod_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for prod in produsers {
        prod_handlers.push(thread::spawn(move || {
            prod.execute();
        }));
    }

    let receiver = Arc::new(Mutex::new(receiver));
    let mut consumers: Vec<Consumer> = vec![];
    for _ in 0..3 {
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
