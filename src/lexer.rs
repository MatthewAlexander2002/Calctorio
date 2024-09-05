use std::vec;
use std::fs::File;
use std::io::prelude::*;
use fsm::nfa::Nfa; //next step i think i should do this in its own file and then bring the NFA here

pub fn lexer(file_loc: &str) -> Vec<String> {
    let mut file = File::open(file_loc).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text: \n{}", contents);

    //probably need a better name for symbols and scope
    let legal_symbols: Vec<&str> = vec!["int", "double", "const", "if", "for", "print", "size", "toINT", "toFloat", "break", "return"];
    let legal_binary_opaterators: Vec<&str> = vec!["&&", "||", "<", ">", "<=", ">=", "<>", "=="];
    let legal_operator: Vec<&str> = vec!["=", "+", "-", "*", "/", "%"];
    let legal_scope: Vec<&str> = vec!["(", ")", "{", "}", "[", "]", ";", "/*", "*/"];
    // let mut token_start = 0;
    let mut current_token = String::new();
    let mut found_tokens: Vec<String> = vec![];

    for (i, c) in contents.chars().enumerate(){

        // print!("{:?}, ", i);
        // println!("{:?}", c);

        if c == '\n' || c == '\t' || c == ' ' {
            if current_token != String::new() {
                println!("{:?}", current_token);
                found_tokens.push(current_token);
                current_token = String::new();
            }
        } else {
            current_token.push(c);
        }
    }
    return found_tokens;
}