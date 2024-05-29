#![allow(unused)]
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // tx.send(val).unwrap();
        // println!("val is {}", val);

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let handle = thread::spawn(move || {
    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        // tx.send(val).unwrap();
        // println!("val is {}", val);

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // handle.join().unwrap();
    // let received = rx.recv().unwrap(); // block the current thread until a message is received
    for received in rx {
        // treated as an iterator, block the current thread until a message is received
        println!("Got: {}", received);
    }
}
