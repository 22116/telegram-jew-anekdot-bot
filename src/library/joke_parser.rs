use async_trait::async_trait;
use rand::seq::SliceRandom;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait Parser {
    async fn parse(&self) -> Result<String, Box<dyn Error>>;
}

pub struct ClassicTravel;

#[async_trait]
impl Parser for ClassicTravel {
    async fn parse(&self) -> Result<String, Box<dyn Error>> {
        let resp = reqwest::get("https://www.classictravel.ru/anekdoty")
            .await?
            .text()
            .await?;

        let document = Html::parse_document(resp.as_str());

        let content_selector = Selector::parse("section.article-content").unwrap();
        let content = document.select(&content_selector).next().unwrap();

        let paragraph_selector = Selector::parse("p").unwrap();
        let paragraphs = content.select(&paragraph_selector);
        let paragraphs: Vec<ElementRef> = paragraphs
            .filter(|&paragraph| !paragraph.html().contains("***"))
            .collect();

        Ok(paragraphs
            .choose(&mut rand::thread_rng())
            .ok_or("No anektod found")?
            .inner_html()
            .replace("<br>", "\n")
            .to_string())
    }
}
