#![allow(warnings)]

use parser::{TreeNode, Symbol, NonTerminal};
use std::fs;
use std::path::Path;
mod lexer;
mod parser;
mod semantic;
mod interpreter;

fn main() {
    // let test_dir = Path::new("/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite");
    // let test_files = find_test_files(test_dir);

    // println!("\nFound test files:");
    // for file in &test_files {
    //     println!("{}", file);
    // }

    // for file in test_files {
    //     println!("\nTesting file: {}", file);
    //     let tokens = lexer::lexer(&file);
    //     match parser::parser(&tokens) {
    //         Ok(tree) => {
    //             println!("Parse successful:");
    //             print_tree(&tree, 0);
    //             semantic::semantic_analysis(tree);
    //         },
    //         Err(e) => println!("Parse error: {}", e),
    //     }
    //     println!("\n--------------------");
    // }

    let file = "/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/InterpreterTests/ManyOps";
    let tokens = lexer::lexer(&file);
        match parser::parser(&tokens) {
            Ok(tree) => {
                println!("Parse successful:");
                print_tree(&tree, 0);
                println!("\n--------------------"); 
                // println!("{:#?}", tree); 
                // match semantic::semantic_analysis(tree) {
                    // Ok(analyzed_tree) => {
                    //     println!("Semantic Analysis successful:");
                    //     print_tree(&analyzed_tree, 0);
                    //     // println!("{:#?}", analyzed_tree);
                 
                interpreter::interpret(&tree);
                    // },
                    // Err(e) => println!("Semantic error: {}", e),
                // }
            },
            Err(e) => println!("Parse error: {}", e),
        }
    println!("\n--------------------"); 
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

fn print_tree(node: &TreeNode, indent: usize) {
    let indent_str = " ".repeat(indent);
    match &node.Symbol {
        Symbol::Terminal(t) => {
            println!("{}Symbol: {:?}", indent_str, t);
        }
        Symbol::NonTerminal(nt) => {
            println!("{}NonTerminal: {:?}", indent_str, nt);
            for child in &node.children {
                print_tree(child, indent + 2);
            }
        }
    }
}