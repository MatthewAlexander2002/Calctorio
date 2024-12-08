#![allow(warnings)]

use parser::{TreeNode, Symbol, NonTerminal};
use std::fs;
use std::path::Path;
mod lexer;
mod parser;
mod semantic;
mod interpreter;

fn main() {
    let test_dirA = Path::new("/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests");
    let test_filesA = find_test_files(test_dirA);

    println!("\nFound test files Atomic:");
    for file in &test_filesA {
        println!("{}", file);
    }

    let test_dirC = Path::new("/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/ComplexTests");
    let test_filesC = find_test_files(test_dirC);

    println!("\nFound test files Complex:");
    for file in &test_filesC {
        println!("{}", file);
    }

    let test_dirI = Path::new("/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/InterpreterTests");
    let test_filesI = find_test_files(test_dirI);

    println!("\nFound test files Interpreted:");
    for file in &test_filesI {
        println!("{}", file);
    }

    println!("ATOMIC TESTS");
    for file in &test_filesA {
        println!("\nTesting file: {}", file);
        let tokens = lexer::lexer(file);
        match parser::parser(&tokens) {
            Ok(tree) => {
                println!("Parse successful:");
                print_tree(&tree, 0);
                println!("\n--------------------"); 
                match semantic::semantic_analysis(&tree) {
                    Ok(symbol_table) => {
                        println!("Semantic Analysis Successful!");
                        println!("Symbol Table: {:?}", symbol_table);
                        println!("\n--------------------");
                        // interpreter::interpret(&tree);
                    }
                    Err(errors) => {
                        println!("Semantic Analysis Failed with Errors:");
                        for error in errors {
                            println!("{}", error.message);
                        }
                    }
                }
            },
            Err(e) => println!("Parse error: {}", e),
        }
        println!("\n--------------------"); 
    }

    println!("COMPLEX TESTS");

    for file in &test_filesC {
        println!("\nTesting file: {}", file);
        let tokens = lexer::lexer(file);
        match parser::parser(&tokens) {
            Ok(tree) => {
                println!("Parse successful:");
                print_tree(&tree, 0);
                println!("\n--------------------"); 
                match semantic::semantic_analysis(&tree) {
                    Ok(symbol_table) => {
                        println!("Semantic Analysis Successful!");
                        println!("Symbol Table: {:?}", symbol_table);
                        println!("\n--------------------");
                        // interpreter::interpret(&tree);
                    }
                    Err(errors) => {
                        println!("Semantic Analysis Failed with Errors:");
                        for error in errors {
                            println!("{}", error.message);
                        }
                    }
                }
            },
            Err(e) => println!("Parse error: {}", e),
        }
        println!("\n--------------------"); 
    }

    println!("INTERPRETER TESTS");

    for file in &test_filesI {
        println!("\nTesting file: {}", file);
        let tokens = lexer::lexer(file);
        match parser::parser(&tokens) {
            Ok(tree) => {
                println!("Parse successful:");
                print_tree(&tree, 0);
                println!("\n--------------------"); 
                match semantic::semantic_analysis(&tree) {
                    Ok(symbol_table) => {
                        println!("Semantic Analysis Successful!");
                        println!("Symbol Table: {:?}", symbol_table);
                        println!("\n--------------------");
                        interpreter::interpret(&tree);
                    }
                    Err(errors) => {
                        println!("Semantic Analysis Failed with Errors:");
                        for error in errors {
                            println!("{}", error.message);
                        }
                    }
                }
            },
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