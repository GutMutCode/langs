#![allow(unused)]
use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn main() {
    // let n = example_closure(5); // type error
    // mutable_borrowed_closure();
    sort();
}

fn shirt_company() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with perference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with perference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn type_inference() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); // type inference
}

fn immutable_borrowed_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrowed_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn move_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // thread::spawn(|| println!("From thread: {:?}", list)) // occurs error if the main thread is
    // ended before the closure
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("After defining closure: {:?}", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("Sorted by width: {:?}", list);

    // let mut sort_oprations = vec![];
    let mut num_sort_operations = 0;
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_oprations.push(value);
        num_sort_operations += 1;
        r.width
    });

    // println!("Sorted by key: {:?}", sort_oprations);
    println!("{:?}, sorted in {num_sort_operations} operations", list);
}
