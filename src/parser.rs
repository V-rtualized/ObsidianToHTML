use crate::models::{Body, Header};
use regex::Regex;

fn text_to_body(text: &str) -> Body {
    Body {
        content: text.trim().to_string(),
    }
}

pub fn split_content(content: &str) -> (Option<Header>, Body) {
    let re = Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---\r?\n(.*)$").unwrap();

    if let Some(captures) = re.captures(content) {
        let yaml_str = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let body_text = captures.get(2).map(|m| m.as_str()).unwrap_or("");

        let header: Option<Header> = serde_yaml::from_str(yaml_str).ok();
        let body = text_to_body(body_text);

        (header, body)
    } else {
        let body = text_to_body(content);
        (None, body)
    }
}
