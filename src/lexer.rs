use std::vec;
use std::fs::File;
use std::io::prelude::*;

//enum token that catagories each enum into sub catagories i.e., operators, types etc.
// then each sub category is a enum within its self

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Type(TypeTK),
    ControlFlow(ControlFlowTK),
    Utilities(UtilitiesTK),
    BinaryOps(BinaryOpsTK),
    Ops(OpsTK),
    Scope(ScopeTK),
    Variable(VariableTK),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeTK { //need both a 
    Int, // int
    Double, // double
    IntVal(i32), // i.e. 8
    DoubleVal(String), // i.e. 8.88 //has to be a string b/c floats don't hash
    Const, // const
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ControlFlowTK {
    If, // if
    For, // for
    While, // while
    Break, // break
    Continue, // continue
    Return, // return
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum UtilitiesTK {
    Print, // print
    Size, // size
    ToINT, // toINT
    ToDouble, // toDOUBLE
    CommentL, // /* 
    CommentR, // */
    SpeechMarks, // "
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum BinaryOpsTK {
    And, // &&
    Or, // ||
    GreaterThan, // >
    LessThan, // <
    GreaterThanEqual, // >=
    LessThanEqual, // <=
    NotEqual, // <>
    Equal, // ==
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum OpsTK {
    Assignment, // =
    Plus, // +
    Minus, // -
    Times, // * 
    Divide, // /
    Modulo, // %
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ScopeTK {
    BracketL, // (
    BracketR, // )
    CurlyBracketL, // {
    CurlyBracketR, // }
    SquareBracketL, // [
    SquareBracketR, // ]
    Semi, // ;
    Comma, // ,
    NewLine, // \r\n //carriage return, new line
    WhiteSpace, // \s " " //white space
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableTK {
    VarName(String),
}


pub fn lexer(file_loc: &str) -> Vec<Token> {
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

    let mut char_iter = contents.chars().peekable();
    let mut current_token = String::new();
    let mut found_tokens: Vec<Token> = vec![];
    // let mut found_tokens: Vec<String> = vec![];

    while let Some(c) = char_iter.next() {
        if c.is_whitespace() {
            if !current_token.is_empty() {
                found_tokens.push(string_to_token(current_token.clone()));
                println!("{:?}", current_token);
                current_token.clear();
            }
            continue;
        }

        if c == '/' && char_iter.peek() == Some(&'/') {
            while let Some(next_char) = char_iter.next() {
                if next_char == '\n' {
                    break;
                }
            }
            continue;
        }

        if c == '/' && char_iter.peek() == Some(&'*') {
            char_iter.next();
            while let Some(next_char) = char_iter.next() {
                if next_char == '*' && char_iter.peek() == Some(&'/') {
                    // Consume the '/' character
                    char_iter.next();
                    break;
                }
            }
            continue;
        }


        //need to change this to check if it has
        if c == '/' && char_iter.peek() == Some(&'*') {
            current_token.push(c);
        } else if c == '*' && current_token == "/" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        } else if c == '*' && char_iter.peek() == Some(&'/') {
            if current_token.is_empty() {
                current_token.push(c);
            } else {
                //turn current word into a token
                found_tokens.push(string_to_token(current_token.clone()));
                println!("{:?}", current_token);
                current_token.clear();
                //then add the soon to be token to the current_token
                current_token.push(c);
            }
        } else if c == '/' && current_token == "*" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        } else if c == '&' && char_iter.peek() == Some(&'&') {
            current_token.push(c);
        } else if c == '&' && current_token == "&" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            current_token.clear();
        } else if c == '|' && char_iter.peek() == Some(&'|') {
            current_token.push(c);
        } else if c == '|' && current_token == "|" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        } else if c == '>' && char_iter.peek() == Some(&'=') {
            current_token.push(c); 
        } else if c == '=' && current_token == ">" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        } else if c == '<' && char_iter.peek() == Some(&'=') {
            current_token.push(c);  
        } else if c == '=' && current_token == "<" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        } else if c == '<' && char_iter.peek() == Some(&'>') {
            current_token.push(c);  
        } else if c == '>' && current_token == "<" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        }  else if c == '=' && char_iter.peek() == Some(&'=') {
            current_token.push(c);  
        } else if c == '=' && current_token == "=" {
            current_token.push(c);
            found_tokens.push(string_to_token(current_token.clone()));
            println!("{:?}", current_token);
            current_token.clear();
        //consider changing this to a .contains on a array but i will refactor later when i figure out whats the faster op
        } else if c == ';' || c == '"' || c == '=' || c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '*' || c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' || c == ',' {
            if c == ' ' && current_token.is_empty() {
                //make sure that leading white space doesn't end up in the next token
                current_token.push(c);
                found_tokens.push(string_to_token(current_token.clone()));
                println!("{:?}", current_token);
                current_token.clear();

            } else if !current_token.is_empty() {
                //turn token up to but not including delimiter into token
                found_tokens.push(string_to_token(current_token.clone()));
                println!("{:?}", current_token);
                current_token.clear();
                
                //turn delimiter into token
                current_token.push(c);
                found_tokens.push(string_to_token(current_token.clone()));
                println!("{:?}", current_token);
                current_token.clear();
            } else {
                current_token.push(c);
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        found_tokens.push(string_to_token(current_token.clone()));
        println!("{:?}", current_token);
    }
    found_tokens.push(Token::EOF);

    return found_tokens;
}

fn string_to_token (current_string: String) -> Token {
    if current_string == "int" { //Types
        return Token::Type(TypeTK::Int);
    } else if current_string == "double" {
        return Token::Type(TypeTK::Double);
    } else if let Ok(int_val) = current_string.parse::<i32>() {
        return Token::Type(TypeTK::IntVal(int_val));
    } else if let Ok(double_val) = current_string.parse::<f32>() {
        return Token::Type(TypeTK::DoubleVal(current_string));
    } else if current_string == "const" {
        return Token::Type(TypeTK::Const);
    } else if current_string == "if" { //Control Flow
        return Token::ControlFlow(ControlFlowTK::If);
    } else if current_string == "for" {
        return Token::ControlFlow(ControlFlowTK::For);
    } else if current_string == "while" {
        return Token::ControlFlow(ControlFlowTK::While);
    } else if current_string == "break" {
        return Token::ControlFlow(ControlFlowTK::Break);
    } else if current_string == "continue" {
        return Token::ControlFlow(ControlFlowTK::Continue);
    } else if current_string == "return" {
        return Token::ControlFlow(ControlFlowTK::Return);
    } else if current_string == "print" { // Utilities
        return Token::Utilities(UtilitiesTK::Print); 
    } else if current_string == "size" {
        return Token::Utilities(UtilitiesTK::Size);
    } else if current_string == "toINT" {
        return Token::Utilities(UtilitiesTK::ToINT);
    } else if current_string == "toDOUBLE" {
        return Token::Utilities(UtilitiesTK::ToDouble);
    } else if current_string == "/*" {
        return Token::Utilities(UtilitiesTK::CommentL);
    } else if current_string == "*/" {
        return Token::Utilities(UtilitiesTK::CommentR);
    } else if current_string == "\"" {
        return Token::Utilities(UtilitiesTK::SpeechMarks);
    } else if current_string == "&&" { // Binary Operators
        return Token::BinaryOps(BinaryOpsTK::And); 
    } else if current_string == "||" {
        return Token::BinaryOps(BinaryOpsTK::Or);
    } else if current_string == ">" {
        return Token::BinaryOps(BinaryOpsTK::GreaterThan);
    } else if current_string == "<" {
        return Token::BinaryOps(BinaryOpsTK::LessThan);
    } else if current_string == ">=" {
        return Token::BinaryOps(BinaryOpsTK::GreaterThanEqual);
    } else if current_string == "<=" {
        return Token::BinaryOps(BinaryOpsTK::LessThanEqual);
    } else if current_string == "<>" {
        return Token::BinaryOps(BinaryOpsTK::NotEqual);
    } else if current_string == "==" {
        return Token::BinaryOps(BinaryOpsTK::Equal);
    } else if current_string == "=" { // Operators
        return Token::Ops(OpsTK::Assignment); 
    } else if current_string == "+" {
        return Token::Ops(OpsTK::Plus);
    } else if current_string == "-" {
        return Token::Ops(OpsTK::Minus);
    } else if current_string == "*" {
        return Token::Ops(OpsTK::Times);
    } else if current_string == "/" {
        return Token::Ops(OpsTK::Divide);
    } else if current_string == "%" {
        return Token::Ops(OpsTK::Modulo);
    } else if current_string == "(" { // Scope
        return Token::Scope(ScopeTK::BracketL);
    } else if current_string == ")" {
        return Token::Scope(ScopeTK::BracketR);
    } else if current_string == "{" {
        return Token::Scope(ScopeTK::CurlyBracketL);
    } else if current_string == "}" {
        return Token::Scope(ScopeTK::CurlyBracketR);
    } else if current_string == "[" {
        return Token::Scope(ScopeTK::SquareBracketL);
    } else if current_string == "]" {
        return Token::Scope(ScopeTK::SquareBracketR);
    } else if current_string == ";" {
        return Token::Scope(ScopeTK::Semi);
    } else if current_string == "," {
        return Token::Scope(ScopeTK::Comma);
    } else {
        return Token::Variable(VariableTK::VarName(current_string)); // Variable Name
    }
}