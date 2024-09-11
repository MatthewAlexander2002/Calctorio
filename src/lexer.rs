use std::vec;
use std::fs::File;
use std::io::{prelude::*, Cursor};

//enum token that catagories each enum into sub catagories i.e., operators, types etc.
// then each sub catgory is a enum within its self

enum Token {
    Type(TypeTK),
    ControlFlow(ControlFlowTK),
    Utilities(UtilitiesTK),
    BinaryOps(BinaryOpsTK),
    Ops(OpsTK),
    Scope(ScopeTK),
    Variable(VariableTK),
}

enum TypeTK { //need both a 
    Int, // int
    Double, // double
    IntVal(i32), // i.e. 8
    DoubleVal(f32), // i.e. 8.88
    Const, // const
}

enum ControlFlowTK {
    If, // if
    For, // for
    While, // while
    Break, // break
    Continue, // continue
    Return, // return
}

enum UtilitiesTK {
    Print, // print
    Size, // size
    ToINT, // toINT
    ToDouble, // toDOUBLE
    CommentL, // /* 
    CommentR, // */
    SpeechMarks, // "
}

enum BinaryOpsTK {
    And, // &&
    Or, // ||
    GreaterThan, // >
    LessThan, // <
    GreaterThanEqual, // >=
    LessThanEqual, // <=
    NotEqual, // <>
    Equal, // ==
}

enum OpsTK {
    Assignment, // =
    Plus, // +
    Minus, // -
    Times, // * 
    Divide, // /
    Modulo, // %
}

enum ScopeTK {
    BracketL, // (
    BracketR, // )
    CurlyBracketL, // {
    CurlyBracketR, // }
    SquareBracketL, // [
    SquareBracketR, // ]
    Semi, // ;
    NewLine, // \r\n //carriage return, new line
    WhiteSpace, // \s " " //white space
}

enum VariableTK {
    VarName(Some),
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
    let mut found_tokens: Vec<Token> = vec![];

    //need to capture "i"(usize) so i can just get a char out
    for (i, c) in contents.chars().enumerate(){

        // print!("{:?}, ", i);
        // println!("{:?}", c);

        //Psudo code idea for my tokenisation
        //if "delimiter"
        // let new_token: Tokens = String_to_Token(current_token: String)
        // found_tokens.push_back(new_token)
        // let new_delim: Tokens = String_to_Token(c: String)
        // found_tokens.push_back(new_delim)
        
        // i think this is the simplest way without going crazy and minmising code repeating

        //end current token on operators, scope, binary operators
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

fn String_to_Token (current_String: String) -> Token {
    if(current_String == "int"){ //Types
        return Token::Type(TypeTK::Int);
    } else if (current_String == "double") {
        return Token::Type(TypeTK::Double);
    } else if (current_String.parse::<i32>().is_ok()) {
        return Token::Type(TypeTK::IntVal(current_String.parse::<i32>()));
    } else if (current_String.parse::<f32>().is_ok()) {
        return Token::Type(TypeTK::IntVal(current_String.parse::<i32>()));
    } else if (current_String == "const") {
        return Token::Type(TypeTK::Const);
    } else if (current_String == "if") { //Control Flow
        return Token::Type(ControlFlowTK::If);
    } else if (current_String == "for") {
        return Token::Type(ControlFlowTK::For);
    } else if (current_String == "while") {
        return Token::Type(ControlFlowTK::While);
    } else if (current_String == "break") {
        return Token::Type(ControlFlowTK::Break);
    } else if (current_String == "continue") {
        return Token::Type(ControlFlowTK::Continue);
    } else if (current_String == "return") {
        return Token::Type(ControlFlowTK::Return);
    } else if (current_String == "print") { // Utilities
        return Token::Type(UtilitiesTK::Print); 
    } else if (current_String == "size") {
        return Token::Type(UtilitiesTK::Size);
    } else if (current_String == "toINT") {
        return Token::Type(UtilitiesTK::ToINT);
    } else if (current_String == "toDOUBLE") {
        return Token::Type(UtilitiesTK::ToDouble);
    } else if (current_String == "/*") {
        return Token::Type(UtilitiesTK::CommentL);
    } else if (current_String == "*/") {
        return Token::Type(UtilitiesTK::CommentR);
    } else if (current_String == "\"") {
        return Token::Type(UtilitiesTK::SpeechMarks);
    } else if (current_String == "&&") { // Binary Operators
        return Token::Type(BinaryOpsTK::And); 
    } else if (current_String == "||") {
        return Token::Type(BinaryOpsTK::Or);
    } else if (current_String == ">") {
        return Token::Type(BinaryOpsTK::GreaterThan);
    } else if (current_String == "<") {
        return Token::Type(BinaryOpsTK::LessThan);
    } else if (current_String == ">=") {
        return Token::Type(BinaryOpsTK::GreaterThanEqual);
    } else if (current_String == "<=") {
        return Token::Type(BinaryOpsTK::LessThanEqual);
    } else if (current_String == "<>") {
        return Token::Type(BinaryOpsTK::NotEqual);
    } else if (current_String == "==") {
        return Token::Type(BinaryOpsTK::Equal);
    } else if (current_String == "=") { // Operators
        return Token::Type(OpsTK::Assignment); 
    } else if (current_String == "+") {
        return Token::Type(OpsTK::Plus);
    } else if (current_String == "-") {
        return Token::Type(OpsTK::Minus);
    } else if (current_String == "*") {
        return Token::Type(OpsTK::Times);
    } else if (current_String == "/") {
        return Token::Type(OpsTK::Divide);
    } else if (current_String == "%") {
        return Token::Type(OpsTK::Modulo);
    } else if (current_String == "(") {
        return Token::Type(ScopeTK::BracketL);
    } else if (current_String == ")") {
        return Token::Type(ScopeTK::BracketR);
    } else if (current_String == "{") {
        return Token::Type(ScopeTK::CurlyBracketL);
    } else if (current_String == "}") {
        return Token::Type(ScopeTK::CurlyBracketR);
    } else if (current_String == "[") {
        return Token::Type(ScopeTK::SquareBracketL);
    } else if (current_String == "]") {
        return Token::Type(ScopeTK::SquareBracketR);
    } else if (current_String == ";") {
        return Token::Type(ScopeTK::Semi);
    } else if (current_String == "\r" || current_String == "\n") {
        return Token::Type(ScopeTK::NewLine);
    } else if (current_String == " ") {
        return Token::Type(ScopeTK::WhiteSpace);
    } else {
        return Token::Type(VariableTK::VarName(current_String));
    }
}