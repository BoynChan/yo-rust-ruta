use crate::error::Error;
use async_trait::async_trait;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use super::Spider;
use crate::entity::news::Entity;
use sea_orm::entity::prelude::DatabaseConnection;
use std::time::Duration;
use time::strptime;

pub struct HackerNewsSpider {
    http_client: Client,
    limit: usize,
    t_list_selector: Selector,
    title_selector: Selector,
    publish_selector: Selector,
    db_pool: DatabaseConnection,
}

#[derive(Debug)]
pub struct New {
    pub title: String,
    pub source: String,
    pub rank: usize,
    pub publish_date: i64,
}
impl HackerNewsSpider {
    pub fn new(limit: usize, db_pool: DatabaseConnection) -> Self {
        let http_timeout = Duration::from_secs(6);
        let http_client = Client::builder()
            .timeout(http_timeout)
            .build()
            .expect("Building HTTP Client");
        let tlist =
            Selector::parse("#hnmain > tbody > tr:nth-child(3) > td > table > tbody > tr.athing")
                .unwrap();
        let title_selc = Selector::parse("span.titleline > a").unwrap();
        let publish_list_selc = Selector::parse(
            "#hnmain > tbody > tr:nth-child(3) > td > table > tbody > tr > td.subtext > span.subline > span.age",
        )
        .unwrap();
        HackerNewsSpider {
            http_client: http_client,
            limit: limit,
            t_list_selector: tlist,
            title_selector: title_selc,
            publish_selector: publish_list_selc,
            db_pool: db_pool,
        }
    }

    fn get_title_timestamp(&self, element: ElementRef) -> i64 {
        let title_str = element.value().attr("title").expect("has title attr");
        let tm = strptime(title_str, "%Y-%m-%dT%H:%M:%S").unwrap();
        tm.to_timespec().sec
    }

    fn get_title(&self, element: ElementRef) -> String {
        HackerNewsSpider::run_selector_get_first(element, &self.title_selector).inner_html()
    }

    fn get_link(&self, element: ElementRef) -> String {
        HackerNewsSpider::run_selector_get_first(element, &self.title_selector)
            .value()
            .attr("href")
            .expect("has no href attr")
            .to_string()
    }

    fn run_selector<'a>(
        element: &'a Html,
        selector: &Selector,
        limit: usize,
    ) -> Vec<ElementRef<'a>> {
        element.select(selector).take(limit).collect()
    }

    fn run_selector_get_first<'a>(element: ElementRef<'a>, selector: &Selector) -> ElementRef<'a> {
        element.select(selector).next().expect("has element")
    }
}

#[async_trait]
impl Spider for HackerNewsSpider {
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
        let mut counter: usize = 1;

        let items_origin =
            HackerNewsSpider::run_selector(&document, &self.t_list_selector, self.limit);
        for element in items_origin {
            let new = New {
                title: self.get_title(element),
                source: self.get_link(element),
                rank: counter,
                publish_date: 0,
            };
            items.push(new);
            counter += 1;
        }
        counter = 1;
        let items_publish_date =
            HackerNewsSpider::run_selector(&document, &self.publish_selector, self.limit);
        for element in items_publish_date {
            items[counter - 1].publish_date = self.get_title_timestamp(element);
            counter += 1;
        }
        Ok((items, vec![]))
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        let mut new = Entity::Model {};
        Ok(())
    }
}
