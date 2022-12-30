use core::hash::Hash;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
fn main() {
    // simulate_expensive_calculation(2);
    generate_workout(3, 15);
    println!("Calculate done")
}

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut catcher = Catcher::new(|num: &u32| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num.to_string()
    });
    if intensity < 25 {
        println!("Intensity < 25:{}", catcher.value(&intensity));
        println!("Calculate twice:{}", catcher.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Random number = 3");
        } else {
            println!("random number != 3 : {}", catcher.value(&intensity))
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct Catcher<'a, T, A, B>
where
    T: Fn(&A) -> B,
    A: Eq + Hash,
{
    calculation: T,
    value: HashMap<&'a A, B>,
}

impl<'a, T, A, B> Catcher<'a, T, A, B>
where
    T: Fn(&A) -> B,
    A: Eq + Hash,
{
    fn new(calculation: T) -> Catcher<'a, T, A, B> {
        Catcher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a A) -> &B {
        let calV = || {
            let v = (self.calculation)(arg);
            return v;
        };
        self.value.entry(arg).or_insert(calV())
    }
}
