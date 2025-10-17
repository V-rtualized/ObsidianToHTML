use crate::models::{Body, Header};

pub fn print_header_information(header: Header) {
    if let Some(tags) = header.tags {
        println!("Tags: {}", tags.join(", "));
    }
    if let Some(aliases) = header.aliases {
        println!("Aliases: {}", aliases.join(", "));
    }
    if let Some(pronunciation) = header.pronunciation {
        println!("Pronunciation: {}", pronunciation);
    }
}

pub fn print_body_information(body: &Body) {
    println!("Lines in body: {}", body.content.lines().count());
    println!("Characters in body: {}", body.content.len());
}
