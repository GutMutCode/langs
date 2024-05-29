#![allow(unused)]

#[derive(Debug)]
pub struct Review {
    content: String,
    // createdAt: Time
}

#[derive(Debug, Default)]
pub struct Store {
    name: String,
    products: Vec<Product>,
}

impl Store {
    // pub fn new() -> Store {
    //     Store {
    //         name: String::from(""),
    //         products: vec![],
    //     }
    // }

    pub fn from(name: &str, products: Vec<Product>) -> Store {
        Store {
            name: name.to_string(),
            products,
        }
    }
}

// impl Default for Store {
//     fn default() -> Self {
//         Self {
//             name: String::from(""),
//             products: vec![],
//         }
//     }
// }

#[derive(Debug)]
pub struct Product {
    name: String,
    price: u64,
    reviews: Vec<Review>,
}
