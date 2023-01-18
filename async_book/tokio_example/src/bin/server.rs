use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Frame,Connection};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String,Bytes>>>;

#[tokio::main]
async fn main() -> Result<(),()>{
    let listener = TcpListener::bind("0.0.0.0:36379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket,_) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket,db).await;
        });
    }
}

async fn process(socket: TcpStream,db:Db){
    use mini_redis::Command::{self,Get,Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()){
                    Frame::Bulk(value.clone())
                }else{
                    Frame::Null
                }
            }
            Set(cmd) => {
                db.lock().unwrap().insert(cmd.key().to_string(),Bytes::from(cmd.value().to_vec()));
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}",cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}