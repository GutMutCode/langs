#![allow(unused)]
use std::rc::Rc;

pub enum Money {
    Debt,
    Savings,
}

pub struct Debt {
    owner: Person,
    borrower: Person,
    kind: Rc<Money>,
}

pub struct Person {
    name: String,
    age: u16,
    asset: Asset,
}

pub struct Asset {}

#[derive(Debug)]
pub struct Company {
    name: String,
    sector: Sector,
}

impl Company {
    pub fn new() -> Company {
        Company {
            name: String::from(""),
            sector: Sector::Bank,
        }
    }

    pub fn from(name: &str, sector: Sector) -> Company {
        Company {
            name: name.to_string(),
            sector,
        }
    }
}

#[derive(Debug)]
pub enum Sector {
    Bank,
}
