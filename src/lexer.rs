use std::vec;
use std::fs::File;
use std::io::prelude::*;

enum Tokens {
    Int(i32), // int
    Double(i32, u32), // double
    Const, // const
    If, // if
    For, // for
    While, // while
    Print, // print
    Size, // size
    ToINT(i32, u32), // toINT
    ToDouble(i32), // toDouble
    Break, // break
    Continue, // continue
    Return(Some), // return
    And, // &&
    Or, // ||
    GreaterThan, // >
    LessThan, // <
    GreaterThanEqual, // >=
    LessThanEqual, // <=
    NotEqual, // <>
    Equal, // ==
    Assignment, // =
    Plus, // +
    Minus, // -
    Times, // * 
    Divide, // /
    Modulo, // %
    BracketL, // (
    BracketR, // )
    CurlyBracketL, // {
    CurlyBracketR, // }
    SquareBracketL, // [
    SquareBracketR, // ]
    Semi, // ;
    CommentL, // /* 
    CommentR, // */
}


pub fn lexer(file_loc: &str) -> Vec<String> {
    let mut file = File::open(file_loc).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text: \n{}", contents);

    //these have been replaced by the enum but until the enum is fin they will be left here 
    //probably need a better name for symbols and scope
    // let legal_symbols: Vec<&str> = vec!["int", "double", "const", "if", "for", "print", "size", "toINT", "toDouble", "break", "return"];
    // let legal_binary_opaterators: Vec<&str> = vec!["&&", "||", "<", ">", "<=", ">=", "<>", "=="];
    // let legal_operator: Vec<&str> = vec!["=", "+", "-", "*", "/", "%"];
    // let legal_scope: Vec<&str> = vec!["(", ")", "{", "}", "[", "]", ";", "/*", "*/"];
    // let mut token_start = 0;
    let mut current_token = String::new();
    let mut found_tokens: Vec<Tokens> = vec![];

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