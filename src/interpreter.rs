use crate::lexer::{self, BinaryOpsTK, ControlFlowTK, OpsTK, ScopeTK, Token, TypeTK, UtilitiesTK, VariableTK};
use crate::parser::{self, NonTerminal, TreeNode, Symbol};
use std::collections::HashMap;
use std::env::var;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref LAST_VAR: Mutex<String> = Mutex::new(String::new());
}

pub fn interpret(tree: &TreeNode) {
    let mut symbol_table: HashMap<String, f64> = HashMap::new(); 
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
            },

            NonTerminal::Statement => {
                if let Symbol::Terminal(Token::Utilities(UtilitiesTK::Print)) = node.Symbol{
                // if is_print_statement(node) {
                    println!("Processing print statement in node: {:?}", node.Symbol);
                    handle_print(node, symbol_table);
                } else {
                    // Process non-print statements
                    for child in &node.children {
                        process_node(child, symbol_table);
                    }
                }
            },

            NonTerminal::VarDecl => {
                let mut var_name = String::new();
                // let mut var_name = node.children[0].children[1].Symbol;
                let mut value = 0.0;

                if let Symbol::Terminal(Token::Variable(VariableTK::VarName(ref name))) = node.children[0].children[1].children[0].Symbol {
                    var_name = name.clone();
                    // println!("{:?}", var_name);
                }
                                                                            //VarDecl, VarDeclP, Ex, BoolEx, RelEx, ArithEx, ArithVal, Number, IntVal
                if let Symbol::Terminal(Token::Type(TypeTK::IntVal(val))) = node.children[1].children[1].children[0].children[0].children[0].children[0].children[0].children[0].Symbol {
                    value = val as f64;
                    // println!("{:?}", value);
                }
                if !var_name.is_empty() && var_name != "main" {
                    symbol_table.insert(var_name.clone(), value); 
                    println!("{:?}", symbol_table);
                    // *LAST_VAR.lock().unwrap() = var_name.clone();
                }
            },
            
            NonTerminal::Assignment => {
                let mut var_name = String::new();
                let mut value = 0.0;

                if let Symbol::Terminal(Token::Variable(VariableTK::VarName(ref name))) = node.children[0].children[0].Symbol {
                    var_name = name.clone();
                    // println!("{:?}", var_name);
                }

                value = evaluate_expression(&node.children[2].children[0].children[0].children[0], symbol_table);

                if !var_name.is_empty() {
                    symbol_table.insert(var_name, value);
                    println!("{:?}", symbol_table);
                }
            },

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

fn search_for_arithVal(node: &TreeNode, symbol_table: &HashMap<String, f64>) -> f64 {
    for child in &node.children {
        match &child.Symbol {
            // Base case: Directly evaluate terminal values
            Symbol::Terminal(token) => match token {
                Token::Type(TypeTK::IntVal(val)) => return *val as f64,
                Token::Type(TypeTK::DoubleVal(val)) => return val.parse::<f64>().unwrap_or(0.0),
                Token::Variable(VariableTK::VarName(name)) => {
                    return *symbol_table.get(name).unwrap_or(&0.0) // Retrieve variable value from the symbol table
                }
                _ => {}
            },
            _ => {
                let result = search_for_arithVal(child, symbol_table);
                if result != 0.0 {
                    return result;
                }
            }
        }
    }
    0.0
}

fn evaluate_expression(node: &TreeNode, symbol_table: &HashMap<String, f64>) -> f64 {
    match &node.Symbol {
        // Base case: Directly evaluate terminal values
        Symbol::Terminal(token) => match token {
            Token::Type(TypeTK::IntVal(val)) => *val as f64,
            Token::Type(TypeTK::DoubleVal(val)) => val.parse::<f64>().unwrap_or(0.0),
            Token::Variable(VariableTK::VarName(name)) => {
                *symbol_table.get(name).unwrap_or(&0.0) // Retrieve variable value from the symbol table
            }
            _ => 0.0,
        },
        // Recursive case: Handle `ArithEx`
        Symbol::NonTerminal(NonTerminal::ArithEx) => {
            let mut result = 0.0;

            for child in &node.children {
                match &child.Symbol {
                    Symbol::NonTerminal(NonTerminal::ArithVal) => {
                        // Recursively evaluate `ArithVal` for the base value
                        result = search_for_arithVal(node, symbol_table);
                        println!("{:?}", result);
                    }
                    Symbol::NonTerminal(NonTerminal::ArithExP) => {
                        // Evaluate the continuation (`ArithExP`)
                        result = evaluate_arith_exp_p(child, result, symbol_table);
                        println!("{:?}", result);
                    }
                    _ => {}
                }
            }

            result
        }
        _ => 0.0,
    }
}

fn evaluate_arith_val(node: &TreeNode, symbol_table: &HashMap<String, f64>) -> f64 {
    for child in &node.children {
        match &child.Symbol {
            Symbol::NonTerminal(NonTerminal::String) => {
                // Drill down into `String → VName → Variable`
                return evaluate_expression(child, symbol_table);
            }
            Symbol::Terminal(token) => match token {
                Token::Type(TypeTK::IntVal(val)) => return *val as f64,
                Token::Type(TypeTK::DoubleVal(val)) => return val.parse::<f64>().unwrap_or(0.0),
                Token::Variable(VariableTK::VarName(name)) => {
                    return *symbol_table.get(name).unwrap_or(&0.0)
                }
                _ => {}
            },
            _ => {}
        }
    }

    0.0 // Default to 0.0 if no value is found
}

// Helper function to process `ArithExP` recursively
fn evaluate_arith_exp_p(node: &TreeNode, accumulated_value: f64, symbol_table: &HashMap<String, f64>) -> f64 {
    let mut result = accumulated_value;
    let mut operation = None;

    for child in &node.children {
        match &child.Symbol {
            // Identify the arithmetic operation
            Symbol::NonTerminal(NonTerminal::ArithOp) => {
                for op_child in &child.children {
                    if let Symbol::Terminal(Token::Ops(op)) = &op_child.Symbol {
                        operation = Some(op);
                    }
                }
            }
            // Evaluate the next `ArithEx`
            Symbol::NonTerminal(NonTerminal::ArithEx) => {
                let value = evaluate_expression(child, symbol_table);
                result = match operation {
                    Some(OpsTK::Plus) => result + value,
                    Some(OpsTK::Minus) => result - value,
                    Some(OpsTK::Times) => result * value,
                    Some(OpsTK::Divide) => result / value,
                    _ => result,
                };
            }
            _ => {}
        }
    }

    result
}
