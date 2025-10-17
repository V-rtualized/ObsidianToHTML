use crate::models::{Body, Header};

fn text_to_body(text: &str) -> Body {
    Body {
        content: text.trim().to_string(),
    }
}

pub fn split_content(content: &str) -> (Option<Header>, Body) {
    if let Some((yaml_str, body_text)) = content
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
    {
        let header: Option<Header> = serde_yaml::from_str(yaml_str).ok();
        let body = text_to_body(body_text);

        return (header, body);
    }

    let body = text_to_body(content);

    (None, body)
}
