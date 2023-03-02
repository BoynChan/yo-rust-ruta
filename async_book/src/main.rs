use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: DB) {
    use mini_redis::Command::{self, Get, Set};
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Set:{:?}", cmd);
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("Get:{:?}", cmd);
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented: {:?}", cmd),
        };
        conn.write_frame(&response).await.unwrap()
    }

    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = Frame::Error("unimplement".to_string());
        conn.write_frame(&response).await.unwrap()
    }
}
