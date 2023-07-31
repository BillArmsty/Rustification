use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //Attempting to use val after we’ve sent it down the channel will give a compile-time error:
        println!("val is {}", val)
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received)
}
