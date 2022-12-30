pub mod garden;
pub mod lib;

use std::cmp::Ordering;
use std::io;

use garden::vegetables;
use lib::front_of_house;
fn main() {
    println!("Hello, world!");
    dbg!(garden::vegetables::Asparagus {
        name: String::from("123")
    });
    vegetables::hi();
    front_of_house::hosting::add_to_waitlist()
}
