#[derive(Debug)]
pub struct Page {
    pub url: String,
    pub content: String,
    pub links: Vec<String>,
}

impl Page {
    pub fn new(url: String, content: String, links: Vec<String>) -> Self {
        Self {
            url,
            content,
            links,
        }
    }
}