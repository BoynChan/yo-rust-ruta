use std::io;
fn main() {
    let a = [1,2,3,4,5];

    let mut search = String::new();

    io::stdin().read_line(&mut search).expect("Failed to read line");

    let index:usize = search.trim().parse().expect("Index was not a number");

    let element = a[index];

    println!("Value of index: {}",element);
}

fn control_flow(mut x:i32) -> i32 {
    let condition = if x > 5 {5} else {0};
    let result = loop {
        x += 1;
        if x > 6 {
            break x
        }
    };

    while x < 10 {
        println!("Value of x {}",x);
    }

    let array = [1,2,3,4,5];
    for a in array {
        println!("Array value {} ",a);
    }
    for a in (1..5).rev() {
        println!("Array value {} ",a);
    }

    6
}
