#![allow(unused)]
use std::slice;

pub fn main() {
    // 19-1, 19-3
    let mut num = 5;

    // let r1 = &num;
    // let r2 = &mut num;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 19-4
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 19-8
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // create raw pointer
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])
    unsafe {
        (
            // dereference raw pointer
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
pub fn main_static1() {
    // 19-9
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
pub fn main_static2() {
    // 19-10
    add_to_count(3);

    unsafe {
        println!("COUNTER is: {}", COUNTER);
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
