use crate::lexer::{self, BinaryOpsTK, ControlFlowTK, OpsTK, ScopeTK, Token, TypeTK, UtilitiesTK, VariableTK};
use crate::parser::{self, NonTerminal, TreeNode, Symbol};

pub fn semantic_analysis(mut root: TreeNode) -> Result<TreeNode, String> {
    // Process the tree recursively
    process_node(&mut root);
    Ok(root)
}

fn process_node(node: &mut TreeNode) {
    let mut new_children = Vec::new();

    for mut child in node.children.drain(..) {
        match child.Symbol {
            Symbol::NonTerminal(_) => {
                for mut grandchild in child.children.drain(..) {
                    process_node(&mut grandchild); 
                    new_children.push(grandchild); 
                }
            }
            Symbol::Terminal(_) => {
                new_children.push(child);
            }
        }
    }
    node.children = new_children;
}
