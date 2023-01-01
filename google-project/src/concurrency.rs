use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

pub fn start_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("Count in main: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn start_scope() {
    let s = String::from("1");
    // std::thread::scope(|scope| {
    //     scope.spawn(|| {
    //         println!("Length: {}", s.len());
    //     })
    // });
}

pub fn channels() {
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received tx2: {:?}", rx.recv());
}

pub fn unbound_channel() {
    let (tx, rs) = mpsc::channel();
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message: {}", i)).unwrap();
            println!("{:?}: sent message", thread_id);
        }
    });
    thread::sleep(Duration::from_millis(100));
    for msg in rs.iter() {
        println!("Recv Msg:{}", msg);
    }
}

pub fn bound_channel() {
    let (tx, rs) = mpsc::sync_channel(3);
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message: {}", i)).unwrap();
            println!("{:?}: sent message", thread_id);
        }
    });
    thread::sleep(Duration::from_millis(100));
    for msg in rs.iter() {
        println!("Recv Msg:{}", msg);
    }
}

pub fn arc() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handlers = Vec::new();
    for _ in 1..5 {
        let v = v.clone();
        handlers.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{:?}: {:?}", thread_id, v)
        }))
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {:?}", v)
}
