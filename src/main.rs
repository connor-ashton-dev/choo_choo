use std::fs;

use crate::parser::tools::Parser;

pub mod dom;
pub mod parser;

fn main() {
    let content = fs::read_to_string("page.html").unwrap();
    let root_node = Parser::parse(content);
    println!("{:#?}", root_node)
}
