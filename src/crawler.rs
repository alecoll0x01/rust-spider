use crate::page::Page;
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::time::Duration;
use url::{ParseError, Url};

pub struct Crawler {
    visited_urls: HashSet<String>,
    pending_urls: VecDeque<String>,
    base_domain: String,
    client: Client,
    max_depth: usize,
}

impl Crawler {
    pub fn new(start_url: &str, max_depth: usize, timeout: u64) -> Result<Self, ParseError> {
        let base_url = Url::parse(start_url)?;
        let domain = base_url.domain().unwrap_or("").to_string();

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();

        Ok(Crawler {
            visited_urls: HashSet::new(),
            pending_urls: VecDeque::from(vec![start_url.to_string()]),
            base_domain: domain,
            client,
            max_depth,
        })
    }

    pub async fn run(&mut self) -> Result<Vec<Page>, Box<dyn Error>> {
        let mut crawled_pages = Vec::new();
        let mut current_depth = 0;

        while let Some(url) = self.pending_urls.pop_front() {
            if current_depth >= self.max_depth {
                break;
            }

            if !self.visited_urls.contains(&url) {
                match self.process_url(&url).await {
                    Ok(page) => {
                        self.visited_urls.insert(url.clone());
                        self.queue_new_urls(&page.links);
                        crawled_pages.push(page);
                    }
                    Err(e) => eprintln!("Error processing {}: {}", url, e),
                }
            }
            current_depth += 1;
        }

        Ok(crawled_pages)
    }

    async fn process_url(&self, url: &str) -> Result<Page, Box<dyn Error>> {
        let content = self.fetch_page(url).await?;
        let links = self.extract_links(&content, url)?;

        Ok(Page::new(url.to_string(), content, links))
    }

    async fn fetch_page(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let content = response.text().await?;
        Ok(content)
    }

    fn extract_links(&self, content: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let document = Html::parse_document(content);
        let selector = Selector::parse("a[href]").unwrap();
        let base = Url::parse(base_url)?;

        let links: Vec<String> = document
            .select(&selector)
            .filter_map(|element| {
                element.value().attr("href").and_then(|href| {
                    Url::parse(href).or_else(|_| base.join(href)).ok()
                })
            })
            .filter(|url| url.domain() == Some(&self.base_domain))
            .map(|url| url.to_string())
            .collect();

        Ok(links)
    }

    fn queue_new_urls(&mut self, urls: &[String]) {
        for url in urls {
            if !self.visited_urls.contains(url) && !self.pending_urls.contains(url) {
                self.pending_urls.push_back(url.clone());
            }
        }
    }
}
