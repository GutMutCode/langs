use std::str;

use money::{Find, People};

fn main() {
    // let bank = Company::from("kakaobank", Sector::Bank);
    // let coupang = Store::from("coupang", vec![]);
    // println!("{:?} : {:?}", bank, coupang);
    let name = String::from("jinho");

    let normal = People::default();
    let p = People::new(name);
    let peoples: Vec<People> = vec![normal];
    // if let is_exist = p.find(peoples) {
    //     println!("{name} people exist: {:?}", p);
    // }
}
