use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // /home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition

    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    
    // println!("{:?}", filename);
    let filename = "/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition";

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text: \n{}", contents);

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
}