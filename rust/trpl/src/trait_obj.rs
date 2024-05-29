#![allow(unused)]
pub trait Draw {
    fn draw(&self);
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// use generic and trait bound
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// use trait object
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw a button
        println!("Drawing a button");
    }
}

pub fn main() {}
