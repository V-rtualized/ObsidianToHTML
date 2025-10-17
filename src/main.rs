use obsidian_to_html::*;
use std::{env::args, fs, process::ExitCode};

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

    if let Some(header_data) = header {
        print_header_information(header_data);
    }

    print_body_information(&body);

    ExitCode::SUCCESS
}
