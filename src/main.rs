use std::sync::mpsc;
use std::thread;

trait Worker {
    fn execute(&self);
}

struct Produser {
    sender: mpsc::Sender<u32>,
}

impl Produser {
    fn new(sender: mpsc::Sender<u32>) -> Produser {
        Produser { sender: sender }
    }
}

impl Worker for Produser {
    fn execute(&self) {
        self.sender.send(5).unwrap();
        println!("Produse!");
    }
}

struct Consumer {
    receiver: mpsc::Receiver<u32>,
}

impl Consumer {
    fn new(receiver: mpsc::Receiver<u32>) -> Consumer {
        Consumer { receiver: receiver }
    }
}

impl Worker for Consumer {
    fn execute(&self) {
        println!("recv {}", self.receiver.recv().unwrap());
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    let mut produsers: Vec<Produser> = vec![];
    for _ in 0..5 {
        let sender = sender.clone();
        produsers.push(Produser::new(sender));
    }

    let mut prod_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for prod in produsers {
        prod_handlers.push(thread::spawn(move || {
            prod.execute();
        }));
    }

    let mut consumers: Vec<Consumer> = vec![];
    for _ in 0..3 {
        let receiver = receiver.clone();
        consumers.push(Consumer::new(receiver));
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
