use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Header {
    pub tags: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub pronunciation: Option<String>,
}

pub struct Body {
    pub content: String,
}
