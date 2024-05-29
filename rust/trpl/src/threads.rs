#![allow(unused)]
use std::thread;
use std::time::Duration;

pub fn intro() {
    // thread::spawn(|| {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // block the main thread until the spawned thread finishes
                            // main thread isn't executed until the spawned thread finishes

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap(); // block the main thread until the spawned thread finishes
}

pub fn main() {
    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
