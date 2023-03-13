use std::borrow::Borrow;

use async_trait::async_trait;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub struct HtmlData {
    pub p_tags: Ptags,
}

#[derive(Debug, Clone, Default)]
pub struct Ptags {
    pub text: Option<Vec<String>>,
}

impl Ptags {
    pub fn new() -> Self {
        Ptags { text: None }
    }
}

pub trait PageContextTrait {
    fn extract_p_tags(&self) -> Result<Option<Ptags>, Box<dyn std::error::Error>>;
    fn extract_image_links(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>;
}
pub struct PageContext {
    pub url: String,
    pub doc: Option<Document>,
    pub image_links: Option<Vec<String>>,
}

impl PageContext {
    fn new(url: &str) -> Self {
        PageContext {
            url: url.to_string(),
            doc: None,
            image_links: None,
        }
    }

    async fn set_html_doc(&mut self) {
        let html = &self.fetch_url().await;
        match html {
            Ok(text) => {
                let doc = Document::from(text.as_str());
                self.doc = Some(doc);
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    async fn fetch_url(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get(&self.url).await?;
        let body = resp.text().await?;
        Ok(body)
    }
}

impl PageContextTrait for PageContext {
    fn extract_p_tags(&self) -> Result<Option<Ptags>, Box<dyn std::error::Error>> {
        let mut p_tag_text: Vec<String> = Vec::new();

        match &self.doc {
            Some(doc) => {
                for node in doc.find(Name("p")) {
                    p_tag_text.push(node.text());
                }
            }
            None => {
                eprintln!("No document initialized");
            }
        }

        Ok(Some(Ptags {
            text: Some(p_tag_text),
        }))
    }

    fn extract_image_links(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let mut image_links: Vec<String> = Vec::new();
        match &self.doc {
            Some(doc) => {
                for node in doc.find(Name("img")) {
                    image_links.push(node.attr("src").unwrap().to_string());
                }
            }
            None => {
                eprintln!("No document");
            }
        }
        Ok(Some(image_links))
    }
}

fn text_cleaner(text: &str) -> String {
    let mut text = text.to_string();
    text = text.replace("\r", "");
    text = text.replace("\t", "");
    text = text.replace("\n", "");
    text = text.trim().to_string();
    return text;
}

#[tokio::main]
async fn main() {
    let input_url = "https://www.dawn.com/news/1741752";
    let mut page_context = PageContext::new(input_url);
    page_context.set_html_doc().await;

    let ptags = page_context.extract_p_tags().unwrap();

    match ptags {
        Some(x) => {
            let ptags = x
                .text
                .unwrap()
                .iter()
                .map(|x| text_cleaner(x))
                .collect::<Vec<String>>();
            println!("{:?}", ptags);
        }
        None => {
            println!("No text");
        }
    }

    let img_links = page_context.extract_image_links().unwrap();

    match img_links {
        Some(x) => {
            let ptags = x.iter().map(|x| text_cleaner(x)).collect::<Vec<String>>();
            println!("{:?}", ptags);
        }
        None => {
            println!("No text");
        }
    }
}
