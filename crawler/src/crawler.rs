use std::{
    collections::HashSet,
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc},
    time::Duration,
};

use futures::StreamExt;
use tokio::{
    sync::{mpsc, Barrier},
    time::sleep,
};

use crate::spider::Spider;

pub struct Crawler {
    delay: Duration,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

impl Crawler {
    // 创建爬虫
    // delay是轮询间隔
    pub fn new(
        delay: Duration,
        crawling_concurrency: usize,
        processing_concurrency: usize,
    ) -> Self {
        Crawler {
            delay,
            crawling_concurrency,
            processing_concurrency,
        }
    }

    /*
    核心Run方法，步骤为：
    1. 创建一些pubsub用于协程之间的通信：
        1. urls_to_visit_channel: 传递需要爬取的url
        2. item_channel: 传递爬取页面之后获取到的内容
        3. new_urls_channel: 一个页面中新爬到的url
    2. 向urls_to_visit_channel注入开始的url
    3. 启动process
        - 处理Item（调用spider的process函数）
    4. 启动scrapers
        - 爬取新页面（调用srcape）
        - 将Item放入item_channel
        - 将新的url放入new_urls_channel
    5. 主循环
        - 将爬取的页面url与已经浏览过的url比较去重
        - 将新的url放入new_urls_channel中爬取
        - 检测是否各个队列为空，是则爬取结束
    */
    //
    pub async fn run<T: Send + 'static>(&self, spider: Arc<dyn Spider<Item = T>>) {
        // page that had been visited. Use a set to record and don't visit it again.
        let mut visited_urls = HashSet::<String>::new();
        let crawling_queue_capacity = self.crawling_concurrency * 400;
        let processing_queue_capacity = self.processing_concurrency * 10;

        let active_spiders = Arc::new(AtomicUsize::new(0));

        // a pubsub
        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawling_queue_capacity);
        let (item_tx, item_rx) = mpsc::channel(processing_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawling_queue_capacity);

        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        self.launch_processors(
            self.processing_concurrency,
            spider.clone(),
            item_rx,
            barrier.clone(),
        );
        self.launch_scrapers(
            self.crawling_concurrency,
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            item_tx,
            active_spiders.clone(),
            self.delay,
            barrier.clone(),
        );

        loop {
            if let Some((visited_url, new_urls)) = new_urls_rx.try_recv().ok() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing:{}", url);
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity
                && urls_to_visit_tx.capacity() == crawling_queue_capacity
                && active_spiders.load(Ordering::SeqCst) == 0
            {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
        log::info!("crawler: control loop exited");
        drop(urls_to_visit_tx);
        barrier.wait().await;
    }

    fn launch_processors<T: Send + 'static>(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        items: mpsc::Receiver<T>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(items)
                .for_each_concurrent(concurrency, |item| async {
                    let _ = spider.process(item).await;
                })
                .await;
            barrier.wait().await;
        });
    }

    fn launch_scrapers<T: Send + 'static>(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        urls_to_visit: mpsc::Receiver<String>,
        new_urls_tx: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<T>,
        active_spiders: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_visit)
                .for_each_concurrent(concurrency, |queued_url| async {
                    active_spiders.fetch_add(1, Ordering::SeqCst);
                    let mut urls = Vec::new();
                    let res = spider
                        .scrape(queued_url.clone().as_str())
                        .await
                        .map_err(|err| {
                            log::error!("{err}");
                            err
                        })
                        .ok();

                    if let Some((items, new_urls)) = res {
                        for item in items {
                            let _ = items_tx.send(item).await;
                        }
                        urls = new_urls;
                    }

                    let _ = new_urls_tx.send((queued_url, urls)).await;
                    sleep(delay).await;
                    active_spiders.fetch_sub(1, Ordering::SeqCst);
                })
                .await;
            drop(items_tx);
            barrier.wait().await;
        });
    }
}
