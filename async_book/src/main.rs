use std::{thread, time};
use futures::executor::block_on;

fn main() {
    let future = doing_sth();
    block_on(future);
    println!("Future is done");
}

async fn doing_sth() {
    thread::sleep(time::Duration::from_millis(1000));
    println!("Hello, world!");
}