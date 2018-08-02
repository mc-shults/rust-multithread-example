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

    for handler in prod_handlers {
        println!("recv: {}", receiver.recv().unwrap());
        handler.join().unwrap();
    }
}
