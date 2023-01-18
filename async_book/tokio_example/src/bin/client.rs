use std::future::join;
use mini_redis::client;

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:36379").await.unwrap();
    let t1 = tokio::spawn(async move {
        client.set("hello", "world".into()).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        let result = client.get("hello").await.unwrap();
        println!("Got value from the server; result={:?}", result);
    });
    t1.await.unwrap();
    t2.await.unwrap();
}
