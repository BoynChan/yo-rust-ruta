use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{self, Hash, Hasher},
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

const NUM_SHARDS: usize = 10;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_sharded_db(NUM_SHARDS);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

type ShardedDB = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDB {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

async fn process(socket: TcpStream, db: ShardedDB) {
    use mini_redis::Command::{self, Get, Set};
    let mut conn = Connection::new(socket);

    let hash_index = |key: String| -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize % NUM_SHARDS
    };

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Set:{:?}", cmd);
                let key = cmd.key().to_string();
                db[hash_index(key.clone())]
                    .lock()
                    .unwrap()
                    .insert(key, cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("Get:{:?}", cmd);
                let key = cmd.key().to_string();
                if let Some(value) = db[hash_index(key.clone())].lock().unwrap().get(cmd.key()) {
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
    // write a spawn thread in tokio
}
