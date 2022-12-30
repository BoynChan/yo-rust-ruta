use rand::Rng;
// use std::cmp::Ordering;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("sn {}", secret_number);

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(len) => println!("Read length:{}", len),
            Err(err) => {
                println!("Input error: {}", err);
                continue;
            }
        };
        println!("Your guessed: {}", guess.trim());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parse error: {}", err);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                return;
            }
        }
    }
}
