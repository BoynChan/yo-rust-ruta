use crate::error::Error;
use async_trait::async_trait;
use std::fmt::Debug;

mod hackernews;
pub use hackernews::HackerNewsSpider;

#[async_trait]
pub trait Spider: Send + Sync {
    type Item;
    fn name(&self) -> String;
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: &str) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
}
