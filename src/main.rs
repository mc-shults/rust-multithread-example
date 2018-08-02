use std::sync::mpsc;
use std::thread;

trait Worker {
    fn execute(&self);
}

struct Produser {}

impl Worker for Produser {
    fn execute(&self) {
        println!("Produse!");
    }
}

fn main() {
    let mut produsers: Vec<Produser> = vec![];
    for _ in 1..3 {
        produsers.push(Produser {});
    }

    let mut prod_handlers: Vec<thread::JoinHandle<()>> = vec![];
    for prod in produsers {
        prod_handlers.push(thread::spawn(move || {
            prod.execute();
        }));
    }

    for handler in prod_handlers {
        handler.join().unwrap();
    }

    //let (sender, receiver) = mpsc::channel();
}
