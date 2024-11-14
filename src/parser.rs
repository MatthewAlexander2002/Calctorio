use crate::lexer;

#[derive(Clone)]
struct TreeNode {
    parent: Option<Box<TreeNode>>,
    children: Vec<TreeNode>,
    value: lexer::Token,
}

impl TreeNode {
    fn new_root(value: lexer::Token) -> Self {
        TreeNode{
            parent: None,
            children: vec![],
            value,
        }
    }

    fn new_child(parent: &mut TreeNode, value: lexer::Token) -> Self {
        let child  = TreeNode{
            parent: Some(Box::new(parent.clone())),
            children: vec![],
            value,
        };
        parent.children.push(child.clone());
        child
    }
}

pub fn parser(tokens: Vec<lexer::Token>) {
    
}