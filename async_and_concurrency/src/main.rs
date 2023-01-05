use std::fmt::Debug;
use std::sync::{Arc, mpsc, Mutex};
use std::{thread, time};
use std::sync::mpsc::Sender;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");
    mutex()
}

fn mutex() {
    let counter = Arc::new(Mutex::new(1));
    let mut handlers = vec![];
    for i_ in 1..10 {
        let c = counter.clone();
        handlers.push(thread::spawn(move || {
            let mut i = c.lock().unwrap();
            *i += 1;
        }))
    }
    for v in handlers {
        v.join().unwrap();
    }
    println!("final value:{:?}",*counter.lock().unwrap())
}

fn channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let start_send = |x:Sender<i32>| {
        thread::spawn(move || {
            let vals = vec![
                1,
                2,
                3,
                4
            ];
            for v in vals {
                x.send(v).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });
    };
    start_send(tx);
    start_send(tx1);

    for received in rx {
        println!("Get Value:{}", received);
    }
}

fn know_move() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        for i in v {
            println!("i:{}", i);
        }
    }).join().unwrap();
}

fn know_join() {
    let join_handler = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} in thread {:?}", thread::current().id());
            thread::sleep(Duration::from_millis(100));
        }
    });
    join_handler.join().unwrap()
}
