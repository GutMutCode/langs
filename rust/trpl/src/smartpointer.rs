#![allow(unused)]
#[derive(Debug)]
pub enum List {
    // enum List<'a> {
    Cons(i32, Box<List>), // compiler can know exact size of this
    // Cons(i32, &'a List<'a>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let a = Cons(3, &Nil);
    // let b = Cons(2, &a);
    // let list = Cons(1, &b);
    // println!("{:?}", list);
    // drop(a);
    // println!("{:?}", list);
    // let a = Box::new(Nil);
    // let b = Box::new(Cons(1, a));
    // let c = Box::new(Cons(2, b));
    // let mut list = Box::new(Cons(3, c));
    // println!("{:?}", list);

    // drop(a);
}

// fn drop(list: List) {
fn drop(list: Box<List>) {
    println!("{:?}", list);
}
