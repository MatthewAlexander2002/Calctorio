#![allow(warnings)]

use parser::{TreeNode, Symbol, NonTerminal};
use std::fs;
use std::path::Path;
mod lexer;
mod parser;
mod semantic;

fn main() {
    let test_dir = Path::new("/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite");
    let test_files = find_test_files(test_dir);

    println!("\nFound test files:");
    for file in &test_files {
        println!("{}", file);
    }

    for file in test_files {
        println!("\nTesting file: {}", file);
        let tokens = lexer::lexer(&file);
        match parser::parser(&tokens) {
            Ok(tree) => println!("Parse successful: {:?}", tree),
            Err(e) => println!("Parse error: {}", e),
        }
        println!("\n--------------------");
    } 
}

fn find_test_files(path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_test_files(&path));
            } else {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }
    files
}