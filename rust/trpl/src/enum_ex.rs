#![allow(dead_code)]
#![allow(unused_variables)]

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    // V4(String), // name(value) : construct instance of Enum
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn def_enum() {
    // let kind = IpAddrKind::V4;
    // println!("{:?}", kind);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    use Coin::*;
    match coin {
        Penny => {
            println!("Lucky Penny!");
            1
        }
        Nickel => 5,
        Dime => 10,
        Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn use_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

pub fn use_lef_if() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is confgured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is confgured to be {}", max);
    }
}

fn get_rest_coin(coin: &Coin) -> u8 {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1
    }
    count
}
