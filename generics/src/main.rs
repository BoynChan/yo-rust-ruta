pub mod lib;
use lib::{Summary, Tweet};

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.1, y: 10.5 };

    let tweet = Tweet {
        username: String::from("house"),
        content: String::from("c"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max_item = list[0];
    for &n in list {
        if n > max_item {
            max_item = n;
        }
    }
    max_item
}
