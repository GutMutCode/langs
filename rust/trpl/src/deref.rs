#![allow(unused)]
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    // type Target = T;
    // fn deref(&self) -> &Self::Target {
    //     &self.0
    // }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x); // copied value of x
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // => *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
