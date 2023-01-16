use tikv_client::{RawClient, Result};
use std::str;

#[tokio::main]
async fn main() ->Result<()>{

    let client = RawClient::new(vec!["10.227.80.91:2379"]).await?;
    client.put("key".to_owned(), "value".to_owned()).await?;
    let value = client.get("key".to_owned()).await?;
    println!("Value in tikv: {}", str::from_utf8(&value.unwrap().to_vec()).unwrap());
    Ok(())
}
