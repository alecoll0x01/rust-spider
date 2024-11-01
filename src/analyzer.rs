use scraper::{Html, Selector};
use std::collections::HashMap;
use crate::page::Page;

pub struct ContentAnalyzer {
    patterns: Vec<AnalysisPattern>,
}

pub struct AnalysisResult {
    url: String,
    findings: HashMap<String, Vec<String>>,
}

struct AnalysisPattern {
    name: String,
    selectors: Vec<String>,
    attributes: Vec<String>,
}

impl ContentAnalyzer {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                AnalysisPattern {
                    name: "login_forms".to_string(),
                    selectors: vec![
                        "form[action*='login']",
                        "form[action*='signin']",
                        "input[type='password']",
                    ].iter().map(|s| s.to_string()).collect(),
                    attributes: vec!["action", "method", "id", "class"].iter().map(|s| s.to_string()).collect(),
                },
                AnalysisPattern {
                    name: "contact_info".to_string(),
                    selectors: vec![
                        "a[href^='mailto:']",
                        "a[href^='tel:']",
                    ].iter().map(|s| s.to_string()).collect(),
                    attributes: vec!["href"].iter().map(|s| s.to_string()).collect(),
                },
                AnalysisPattern {
                    name: "social_media".to_string(),
                    selectors: vec![
                        "a[href*='facebook.com']",
                        "a[href*='twitter.com']",
                        "a[href*='linkedin.com']",
                        "a[href*='instagram.com']",
                    ].iter().map(|s| s.to_string()).collect(),
                    attributes: vec!["href"].iter().map(|s| s.to_string()).collect(),
                },
            ],
        }
    }

    pub fn analyze(&self, page: &Page) -> AnalysisResult {
        let document = Html::parse_document(&page.content);
        let mut findings = HashMap::new();

        for pattern in &self.patterns {
            let mut pattern_findings = Vec::new();

            for selector_str in &pattern.selectors {
                if let Ok(selector) = Selector::parse(selector_str) {
                    for element in document.select(&selector) {
                        let mut element_data = Vec::new();
                        
                        for attr in &pattern.attributes {
                            if let Some(value) = element.value().attr(attr) {
                                element_data.push(format!("{}: {}", attr, value));
                            }
                        }

                        if let Some(text) = element.text().next() {
                            if !text.trim().is_empty() {
                                element_data.push(format!("text: {}", text.trim()));
                            }
                        }

                        if !element_data.is_empty() {
                            pattern_findings.push(element_data.join(", "));
                        }
                    }
                }
            }

            if !pattern_findings.is_empty() {
                findings.insert(pattern.name.clone(), pattern_findings);
            }
        }

        AnalysisResult {
            url: page.url.clone(),
            findings,
        }
    }
}

impl AnalysisResult {
    pub fn patterns_found(&self) -> usize {
        self.findings.len()
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("URL: {}\n\n", self.url);

        for (pattern, findings) in &self.findings {
            result.push_str(&format!("=== {} ===\n", pattern));
            for finding in findings {
                result.push_str(&format!("- {}\n", finding));
            }
            result.push('\n');
        }

        result
    }
}
