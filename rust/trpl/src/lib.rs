#![allow(unused_variables)]
#![allow(dead_code)]

mod closures;
mod errors;
mod front_of_house;
mod hash_map;

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     //Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist(); // use short-hand syntax
}

mod customer {
    // this won't compile - (different scope)
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
