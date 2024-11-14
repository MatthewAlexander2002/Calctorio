#![allow(warnings)]

use crate::lexer::{self, Token, TypeTK};
use std::collections::{hash_map, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NonTerminal {
    Prog,
    FuncList,
    FuncDecl,
    ListOfParams,
    NonEmptyListOfParams,
    NonEmptyListOfParamsContinue,
    StatementList,
    Statement,
    ForLoopFirstBit,
    ForLoopMiddleBit,
    ForLoopLastBit,
    ReturnTail,
    Text,
    TextElement,
    TextTail,
    Assignment,
    VarDecl,
    VarDeclP,
    Decl,
    Ex,
    BoolEx,
    BoolExP,
    BoolOp,
    RelEx,
    RelExP,
    RelOp,
    ArithEx,
    ArithExP,
    ArithOp,
    ArithOpP,
    ArithVal,
    String,
    FnCall,
    ArgList,
    ArgListTail,
    Type,
    VName,
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Production {
    Rule(Vec<Symbol>),
    Epsilon
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    Terminal(Token),
    NonTerminal(NonTerminal),
}

// #[derive(Clone)]
// pub struct TreeNode {
//     // parent: Option<Box<TreeNode>>,
//     children: Vec<TreeNode>,
//     name: String,
// }

fn build_parse_table() -> HashMap<(NonTerminal, Token), Production> {
    let mut table: HashMap<(NonTerminal, Token), Production> = HashMap::new();

    table.insert((NonTerminal::Prog, Token::EOF), Production::Epsilon);

    table.insert((NonTerminal::FuncList, Token::EOF), Production::Epsilon);

    table.insert(
        (NonTerminal::FuncList, Token::Type(TypeTK::Int)), 
        Production::Rule(vec![
            Symbol::NonTerminal(NonTerminal::FuncDecl), 
            Symbol::NonTerminal(NonTerminal::FuncList),    
        ]),
    );

    table
} 

// impl TreeNode {
//     fn prog() -> Self {
//         TreeNode{
//             parent: None,
//             children: vec![],
//             name: String::from("Prog"),
//         }
//     }

//     fn func_list(parent: &mut TreeNode) -> Self {
//         TreeNode{
//             parent: Some(Box::new(parent.clone())),
//             children: vec![],
//             name: String::from("FuncList"),
//         }
//     }




//     fn new_child(parent: &mut TreeNode, name: String) -> Self {
//         let child  = TreeNode{
//             parent: Some(Box::new(parent.clone())),
//             children: vec![],
//             name,
//         };
//         parent.children.push(child.clone());
//         child
//     }


// }

// pub fn parser(tokens: Vec<lexer::Token>) -> TreeNode {
//     let mut table: HashMap<(lexer::Token, String), TreeNode> = HashMap::new();

//     let mut root = TreeNode{ parent: None, children: vec![TreeNode{ parent: None, children: vec![], name: String::from("FuncList"), value: lexer::Token::Entry}], name: String::from("Prog"), value: lexer::Token::Entry};
//     let mut func_list = TreeNode{ parent: None, children: vec![TreeNode{ parent: None, children: vec![], name: String::from("FuncDecl"), value: lexer::Token::Entry}, func_list], name: String::from("FuncList"), value: lexer::Token::Entry};
//     let mut func_decl = TreeNode{ parent: None, children: vec![], name: String::from("FuncDecl"), value: lexer::Token::Entry};


//     let root = TreeNode::new_root(tokens[0].clone());
//     // let func_list = TreeNode::new_child(&mut root, Token::Entry, String::from("FuncList"));
//     table.insert((Token::Entry, String::from("Prog")), TreeNode::new_root(tokens[0].clone()));

//     // table.insert((Token::Entry, String::from("FuncList")), );

//     root
// }