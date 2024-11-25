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
        // Recursively process the child node
        process_node(&mut child);

        match child.Symbol {
            Symbol::NonTerminal(_) => {
                // Only retain children of the non-terminal in the tree
                new_children.extend(child.children.drain(..));
            }
            Symbol::Terminal(_) => {
                // Keep terminal nodes as they are
                new_children.push(child);
            }
        }
    }

    // Replace the current node's children with the updated list
    node.children = new_children;
}
