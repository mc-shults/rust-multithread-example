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

fn run<T>(obj: T)
// -> thread::JoinHandle<()>
where
    T: Worker + Send,
{
    obj.execute();
    //thread::spawn(move || {
    //    obj.execute();
    //})
}

fn say_hello() {
    println!("Hello, world!")
}

fn main() {
    say_hello();
    println!("Hello, world!");
    let child = thread::spawn(move || {
        println!("New thread :)");
    });
    child.join().unwrap();
    let p = Produser {};

    let child1 = thread::spawn(move || {
        p.execute();
    });
    child1.join().unwrap();
}
