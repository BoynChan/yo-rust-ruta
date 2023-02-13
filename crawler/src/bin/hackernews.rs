use crawler::db_instance;
use crawler::spider::HackerNewsSpider;
use crawler::Crawler;
use crawler::Error;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_pool = db_instance().await?;
    let spider = Arc::new(HackerNewsSpider::new(10, db_pool));
    let crawler = Crawler::new(Duration::from_millis(200), 2, 500);
    crawler.run(spider).await;
    Ok(())
}
