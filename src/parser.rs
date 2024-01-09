use scraper::{Html, Selector};

use crate::BoxResult;

#[derive(Debug)]
pub(crate) struct Info {
    pub(crate) title: Option<String>,
    pub(crate) img_urls: Vec<Option<String>>,
}

pub(crate) fn parse<B>(body: B) -> BoxResult<Info>
where
    B: AsRef<str>,
{
    let doc = Html::parse_document(body.as_ref());
    let title_selector = Selector::parse("title")?;
    let img_selector = Selector::parse("img")?;

    let title = doc.select(&title_selector).next().map(|e| e.inner_html());

    let img_urls = doc
        .select(&img_selector)
        .map(|e| e.value().attr("src").map(|s| s.to_owned()))
        .collect();

    Ok(Info { title, img_urls })
}
