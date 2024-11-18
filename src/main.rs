#![allow(warnings)]
mod lexer;
mod parser;

fn main() {
    // /home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition

    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    
    // println!("{:?}", filename);
    let filename = "/home/matthew/Documents/UNI/Sem 6/SDL/Calctorio/TestSuite/AtomicTests/Addition";
    let tokens = lexer::lexer(filename);
    let root = parser::parser(&tokens);
    print_tree(&root, 0);
}

fn print_tree(node: &parser::TreeNode, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("{:?}", node);
    for child in &node.children {
        print_tree(&child.borrow(), depth + 1);
    }
}