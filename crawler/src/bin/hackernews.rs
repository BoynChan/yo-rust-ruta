use crawler::spider::HackerNewsSpider;
use crawler::Crawler;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let spider = Arc::new(HackerNewsSpider::new(10));
    let crawler = Crawler::new(Duration::from_millis(200), 2, 500);
    crawler.run(spider).await;
}
