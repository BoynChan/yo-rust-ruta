use bytes::Bytes;
use mini_redis::{client, cmd, Command, Result};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum AsyncCommand {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                AsyncCommand::Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).unwrap();
                }
                AsyncCommand::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res).unwrap();
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = AsyncCommand::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res)
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = AsyncCommand::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res)
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

    // let mut client = client::connect("127.0.0.1:6379").await?;
    // let t1 = tokio::spawn(async {
    //     let res = client.get("foo").await;
    // });
    // let t2 = tokio::spawn(async {
    //     let res = client.set("foo", "bar".into()).await;
    // });
    // t1.await.unwrap();
    // t2.await.unwrap();
    Ok(())
}
