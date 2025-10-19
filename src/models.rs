use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Header {
    pub tags: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub pronunciation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub content: String,
}
