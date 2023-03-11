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
    fn extract_p_tags(&self, url: &str) -> Result<Option<Ptags>, Box<dyn std::error::Error>>;
}
pub struct PageContext {
    pub url: String,
    pub doc: Option<Document>,
}

impl PageContext {
    fn new(url: &str) -> Self {
        PageContext {
            url: url.to_string(),
            doc: None,
        }
    }

    fn set_html_doc(&mut self) {
        let doc = Document::from(self.url.as_str());
        self.doc = Some(doc);
    }
}

impl PageContextTrait for PageContext {
    fn extract_p_tags(&self, html_text: &str) -> Result<Option<Ptags>, Box<dyn std::error::Error>> {
        let doc = Document::from(html_text);
        let mut p_tag_text: Vec<String> = Vec::new();

        for node in doc.find(Name("p")) {
            p_tag_text.push(node.text());
        }

        Ok(Some(Ptags {
            text: Some(p_tag_text),
        }))
    }
}

async fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
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
    let input_url = "https://www.rust-lang.org/";
    let text = fetch_url(input_url).await;
    let mut page_context = PageContext::new(input_url);
    page_context.set_html_doc();

    let mut ptags = Some(Ptags::new());
    match text {
        Ok(x) => {
            ptags = page_context.extract_p_tags(&x).unwrap();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

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

    let text = print!("Fetching {}...", input_url);

    println!("Hello, world!");
}
