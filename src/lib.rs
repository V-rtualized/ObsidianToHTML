mod display;
mod models;
mod parser;

pub use display::{print_body_information, print_header_information};
pub use models::{Body, Header};
pub use parser::split_content;
