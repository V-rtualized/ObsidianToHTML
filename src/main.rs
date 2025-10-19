use clap::Parser;
use obsidian_to_html::*;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "obsidian-to-html")]
#[command(about = "Parse Obsidian markdown files", long_about = None)]
struct Args {
    /// Path to the Obsidian markdown file
    file_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file_name)?;

    let (header, body) = split_content(&content);

    if let Some(header_data) = header {
        print_header_information(header_data);
    }

    print_body_information(&body);

    Ok(())
}
