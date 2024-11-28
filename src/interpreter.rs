use crate::lexer::{self, BinaryOpsTK, ControlFlowTK, OpsTK, ScopeTK, Token, TypeTK, UtilitiesTK, VariableTK};
use crate::parser::{self, NonTerminal, TreeNode, Symbol};
use std::collections::HashMap;

pub fn interpret(tree: &TreeNode) {
    let mut symbol_table = HashMap::new(); 
    process_node(tree, &mut symbol_table);
}

fn process_node(node: &TreeNode, symbol_table: &mut HashMap<String, f64>) {
    println!("Processing node: {:?}", node.Symbol); // Debugging output

    match &node.Symbol {
        Symbol::NonTerminal(non_terminal) => match non_terminal {
            NonTerminal::Prog | NonTerminal::FuncDecl | NonTerminal::StatementList => {
                // Process all children in the list
                for (i, child) in node.children.iter().enumerate() {
                    println!("Processing child {} of StatementList: {:?}", i, child.Symbol);
                    process_node(child, symbol_table);
                }
            }
            NonTerminal::Statement => {
                if is_print_statement(node) {
                    println!("Processing print statement in node: {:?}", node.Symbol);
                    handle_print(node, symbol_table);
                } else {
                    // Process non-print statements
                    for child in &node.children {
                        process_node(child, symbol_table);
                    }
                }
            }
            NonTerminal::Decl => {
                let mut var_name = String::new();
                for child in &node.children {
                    if let Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) = &child.Symbol {
                        var_name = name.clone();
                    }
                }
                if !var_name.is_empty() {
                    symbol_table.insert(var_name, 0.0); // Initialize to 0.0
                }
            }
            NonTerminal::Assignment => {
                let mut var_name = String::new();
                let mut value = 0.0;
                for child in &node.children {
                    match &child.Symbol {
                        Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) => {
                            var_name = name.clone();
                        }
                        _ => {
                            value = evaluate_expression(child, symbol_table);
                        }
                    }
                }
                if !var_name.is_empty() {
                    symbol_table.insert(var_name, value);
                }
            }
            _ => {
                for child in &node.children {
                    process_node(child, symbol_table);
                }
            }
        },
        _ => {}
    }
}

fn is_print_statement(node: &TreeNode) -> bool {
    // Check for Utilities(Print) at any level in the children
    for child in &node.children {
        if let Symbol::Terminal(Token::Utilities(UtilitiesTK::Print)) = child.Symbol {
            println!("Found print statement in node: {:?}", node.Symbol);
            return true;
        }
    }
    false
}



fn handle_print(node: &TreeNode, symbol_table: &HashMap<String, f64>) {
    // Find and evaluate the `Text` or related content to print
    for child in &node.children {
        match &child.Symbol {
            Symbol::NonTerminal(NonTerminal::Text) | Symbol::NonTerminal(NonTerminal::TextElement) => {
                let value_to_print = evaluate_text(child, symbol_table);
                println!("Print value: {}", value_to_print);
            }
            _ => {} // Ignore other parts (e.g., parentheses or semicolon)
        }
    }
}



fn evaluate_text(node: &TreeNode, symbol_table: &HashMap<String, f64>) -> String {
    match &node.Symbol {
        Symbol::NonTerminal(NonTerminal::VName) => {
            if let Some(child) = node.children.first() {
                if let Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) = &child.Symbol {
                    // Retrieve the variable value from the symbol table
                    return format!("{}", symbol_table.get(name).unwrap_or(&0.0));
                }
            }
        }
        Symbol::Terminal(Token::Type(TypeTK::IntVal(val))) => {
            return format!("{}", val);
        }
        Symbol::Terminal(Token::Type(TypeTK::DoubleVal(val))) => {
            return val.clone();
        }
        _ => {
            // For other cases, recursively evaluate the children
            let mut result = String::new();
            for child in &node.children {
                result += &evaluate_text(child, symbol_table);
            }
            return result;
        }
    }

    String::new()
}

fn evaluate_expression(node: &TreeNode, symbol_table: &HashMap<String, f64>) -> f64 {
    match &node.Symbol {
        Symbol::Terminal(token) => match token {
            Token::Type(TypeTK::IntVal(val)) => *val as f64,
            Token::Type(TypeTK::DoubleVal(val)) => val.parse::<f64>().unwrap_or(0.0),
            Token::Variable(VariableTK::VarName(name)) => {
                *symbol_table.get(name).unwrap_or(&0.0) // Retrieve variable value
            }
            _ => 0.0,
        },
        Symbol::NonTerminal(non_terminal) => match non_terminal {
            NonTerminal::ArithEx | NonTerminal::RelEx | NonTerminal::BoolEx => {
                let mut result = 0.0;
                let mut operation = None;

                for child in &node.children {
                    match &child.Symbol {
                        Symbol::Terminal(Token::Ops(op)) => {
                            operation = Some(op);
                        }
                        _ => {
                            let value = evaluate_expression(child, symbol_table);
                            result = match operation {
                                Some(OpsTK::Plus) => result + value,
                                Some(OpsTK::Minus) => result - value,
                                Some(OpsTK::Times) => result * value,
                                Some(OpsTK::Divide) => result / value,
                                _ => value, // Default for the first operand
                            };
                        }
                    }
                }
                result
            }
            _ => 0.0,
        },
        _ => 0.0,
    }
}