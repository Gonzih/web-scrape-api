use crabquery::*;
use serde_derive::{Serialize};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Serialize, Debug)]
pub struct ElementData {
    tag: Option<String>,
    source_url: Option<String>,
    text: Option<String>,
    attrs: HashMap<String, Option<String>>,
}

impl From<&Element> for ElementData {
    fn from(e: &Element) -> Self {
        Self {
            tag: e.tag(),
            source_url: None,
            text: e.text(),
            attrs: HashMap::new(),
        }
    }
}

pub struct Scraper {
    selectors: Vec<String>,
    urls: Vec<String>,
    attrs: Vec<String>,
}

impl Scraper {
    pub fn new(selectors: Vec<String>, urls: Vec<String>, attrs: Vec<String>) -> Self {
        Self { selectors, urls, attrs }
    }

    pub async fn elements(&self) -> Result<Vec<ElementData>> {
        let mut result = vec![];

        let handles: Vec<_> = self.urls.iter().map(|url| (url, reqwest::get(url))).collect();

        for response in handles {
            let (url, response) = response;
            let document = Document::from(response.await?.text().await?);

            for selector in &self.selectors {
                let els = document.select(selector);
                for el in els {
                    let mut e = ElementData::from(&el);
                    e.source_url = Some(url.to_string());

                    for attr in &self.attrs {
                        let v = el.attr(attr);
                        if let Some(_) = v {
                            e.attrs.insert(attr.to_string(), v);
                        }
                    }

                    result.push(e);
                }
            }
        }

        Ok(result)
    }
}
