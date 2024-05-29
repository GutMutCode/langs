// #![allow(dead_code)]
#[allow(unused)]
enum Things {}

#[derive(Default, Debug, Clone)]
pub enum People {
    #[default]
    Normal,
    Mutant(String),
}

impl People {
    pub fn new(name: String) -> Self {
        Self::Mutant(name)
    }
}

pub trait Find {
    fn find<T>(&self, range: &Vec<T>) -> Result<&T, ()>;
    // where
    //     T: PartialEq;
}

// TODO: I don't know how to implement find elements from vector
#[allow(unused)]
impl Find for People {
    fn find<People>(&self, range: &Vec<People>) -> Result<People, ()> {
        match range.first() {
            Some(x) => Ok(x.clone()),
            None => Err(()),
        }
    }
    // fn find<People>(&self, range: &Vec<People>) -> Option<&People> {
    //     range.first()
    // }
}
