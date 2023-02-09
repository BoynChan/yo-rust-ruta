use async_trait::async_trait;
use crawler::error::Error;
use crawler::Crawler;
use reqwest::Client;
use scraper::{Html, Selector};

use std::sync::Arc;
use std::time::Duration;

pub struct HackerNewsSpider {
    http_client: Client,
    limit: usize,
}

#[derive(Debug)]
pub struct New {
    title: String,
    source: String,
    rank: i32,
    publish_date: i64,
    link: String,
}

impl HackerNewsSpider {
    pub fn new(limit: usize) -> Self {
        let http_timeout = Duration::from_secs(6);
        let http_client = Client::builder()
            .timeout(http_timeout)
            .build()
            .expect("Building HTTP Client");
        HackerNewsSpider {
            http_client: http_client,
            limit: limit,
        }
    }
}

#[async_trait]
impl crawler::spider::Spider for HackerNewsSpider {
    type Item = New;

    fn name(&self) -> String {
        String::from("HackerNews")
    }

    fn start_urls(&self) -> Vec<String> {
        vec!["https://news.ycombinator.com/".to_string()]
    }

    async fn scrape(&self, url: &str) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        log::info!("visiting: {}", url);
        let http_res = self.http_client.get(url).send().await?.text().await?;
        let mut items = Vec::new();
        let document = Html::parse_document(http_res.as_str());
        let tlist = Selector::parse(
            "#hnmain > tbody > tr:nth-child(3) > td > table > tbody > tr[class='athing']",
        )
        .unwrap();
        let itemsOrigin: Vec<scraper::ElementRef> =
            document.select(&tlist).take(self.limit).collect();
        for element in itemsOrigin {
            // println!("element:{}", element.inner_html());
            items.push(New {
                title: String::from(""),
                source: String::from(""),
                rank: 0,
                publish_date: 0,
                link: String::from(""),
            })
        }
        Ok((items, vec![]))
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        // println!("{:?}", item);

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let spider = Arc::new(HackerNewsSpider::new(10));
    let crawler = Crawler::new(Duration::from_millis(200), 2, 500);
    crawler.run(spider).await;
}
