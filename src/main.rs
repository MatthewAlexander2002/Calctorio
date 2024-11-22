#![allow(warnings)]

use parser::{TreeNode, Symbol, NonTerminal};
mod lexer;
mod parser;

fn main() {
    // /home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition

    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    
    // println!("{:?}", filename);
    let filename = "/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition";
    let tokens = lexer::lexer(filename);

    // parser::parser(&tokens);

    match parser::parser(&tokens) {
        Ok(tree) => println!("Parse successful: {:?}", tree),
        Err(e) => println!("Parse error: {}", e),
    }
    
}

// fn print_tree(node: &parser::TreeNode, depth: usize) {
//     for _ in 0..depth {
//         print!("  ");
//     }
//     println!("{:?}", node);
//     for child in &node.children {
//         print_tree(&child.borrow(), depth + 1);
//     }
// }