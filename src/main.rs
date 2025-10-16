use serde::Deserialize;
use std::{env::args, fs, process::ExitCode};

#[derive(Debug, Deserialize)]
struct Header {
    tags: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
    pronunciation: Option<String>,
}

struct Body {
    content: String,
}

fn text_to_body(text: &str) -> Body {
    Body {
        content: text.to_string(),
    }
}

fn split_content(content: &str) -> (Option<Header>, Body) {
    if let Some((yaml_str, body_text)) = content
        .strip_prefix("---")
        .and_then(|rest| rest.split_once("---"))
    {
        let header: Option<Header> = serde_yaml::from_str(yaml_str).ok();
        let body = text_to_body(body_text);

        return (header, body);
    }

    let body = text_to_body(content);

    (None, body)
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    let program_name = &args[0];

    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", program_name);
        return ExitCode::FAILURE;
    }

    let file_name = &args[1];

    let content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", file_name, e);
            return ExitCode::FAILURE;
        }
    };

    println!("File: {}", file_name);

    let (header, body) = split_content(&content);

    match header {
        Some(header_data) => {
            if let Some(tags) = header_data.tags {
                println!("Tags: {}", tags.join(", "));
            }
            if let Some(aliases) = header_data.aliases {
                println!("Aliases: {}", aliases.join(", "));
            }
            if let Some(pronunciation) = header_data.pronunciation {
                println!("Pronunciation: {}", pronunciation);
            }
        }
        None => {
            println!("File has no header")
        }
    }

    println!("Lines in body: {}", body.content.lines().count());
    println!("Characters in body: {}", body.content.len());

    ExitCode::SUCCESS
}
