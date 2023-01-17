use tokio::task::yield_now;
use std::rc::Rc;
use std::sync::Arc;

#[tokio::main]
async fn main() ->Result<(),()> {
    let join = tokio::spawn(async {
        // Line below will failed to compile because Rc is not Send
        // let rc = Rc::new("Hello");

        // We should use Arc in order to send from different thread
        let rc = Arc::new("Hello");
        yield_now().await;
        println!("{}", rc);
    });
    join.await.unwrap();
    Ok(())
}