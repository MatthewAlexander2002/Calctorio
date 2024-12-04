use crate::lexer::{self, BinaryOpsTK, ControlFlowTK, OpsTK, ScopeTK, Token, TypeTK, UtilitiesTK, VariableTK};
use crate::parser::{self, NonTerminal, TreeNode, Symbol};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
}

pub type SymbolTable = HashMap<String, String>; // Variable name -> Type

pub fn semantic_analysis(tree: &TreeNode) -> Result<SymbolTable, Vec<SemanticError>> {
    let mut symbol_table: SymbolTable = HashMap::new();
    let mut errors = Vec::new();

    analyze_node(tree, &mut symbol_table, &mut errors);

    if errors.is_empty() {
        Ok(symbol_table)
    } else {
        Err(errors)
    }
}

fn analyze_node(node: &TreeNode, symbol_table: &mut SymbolTable, errors: &mut Vec<SemanticError>) {
    match &node.Symbol {
        Symbol::NonTerminal(non_terminal) => match non_terminal {
            NonTerminal::Prog => {
                for child in &node.children {
                    analyze_node(child, symbol_table, errors);
                }
            }
            NonTerminal::StatementList => {
                for child in &node.children {
                    analyze_node(child, symbol_table, errors);
                }
            }
            NonTerminal::Decl => {
                analyze_variable_declaration(node, symbol_table, errors);
            }
            NonTerminal::Assignment => {
                analyze_assignment(node, symbol_table, errors);
            }
            _ => {
                for child in &node.children {
                    analyze_node(child, symbol_table, errors);
                }
            }
        },
        _ => {}
    }
}

fn analyze_variable_declaration(node: &TreeNode, symbol_table: &mut SymbolTable, errors: &mut Vec<SemanticError>) {
    let mut var_name = String::new();
    let mut var_type = String::new();

    for child in &node.children {
        match &child.Symbol {
            Symbol::NonTerminal(NonTerminal::Type) => {
                if let Some(child_node) = child.children.first() {
                    if let Symbol::Terminal(Token::Type(type_token)) = &child_node.Symbol {
                        var_type = format!("{:?}", type_token).to_lowercase(); // Normalize to lowercase
                    }
                }
            }
            Symbol::NonTerminal(NonTerminal::VName) => {
                if let Some(child_node) = child.children.first() {
                    if let Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) = &child_node.Symbol {
                        var_name = name.clone();
                    }
                }
            }
            _ => {}
        }
    }

    if var_name.is_empty() {
        errors.push(SemanticError {
            message: "Variable declaration missing name.".to_string(),
        });
    } else if symbol_table.contains_key(&var_name) {
        errors.push(SemanticError {
            message: format!("Variable '{}' is already declared.", var_name),
        });
    } else {
        symbol_table.insert(var_name, var_type);
    }
}

fn analyze_assignment(node: &TreeNode, symbol_table: &mut SymbolTable, errors: &mut Vec<SemanticError>) {
    let mut var_name = String::new();
    let mut assigned_type = String::new();

    for child in &node.children {
        match &child.Symbol {
            Symbol::NonTerminal(NonTerminal::VName) => {
                if let Some(child_node) = child.children.first() {
                    if let Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) = &child_node.Symbol {
                        var_name = name.clone();
                    }
                }
            }
            Symbol::NonTerminal(NonTerminal::Ex) => {
                assigned_type = evaluate_expression_type(child, symbol_table, errors).to_lowercase();
            }
            _ => {}
        }
    }

    if var_name.is_empty() {
        errors.push(SemanticError {
            message: "Assignment is missing a target variable.".to_string(),
        });
    } else if !symbol_table.contains_key(&var_name) {
        errors.push(SemanticError {
            message: format!("Assignment to undeclared variable '{}'.", var_name),
        });
    } else if let Some(declared_type) = symbol_table.get(&var_name) {
        if declared_type.to_lowercase() != assigned_type {
            errors.push(SemanticError {
                message: format!(
                    "Type mismatch in assignment: variable '{}' is of type '{}' but assigned type '{}'.",
                    var_name, declared_type, assigned_type
                ),
            });
        }
    }
}

fn evaluate_expression_type(node: &TreeNode, symbol_table: &SymbolTable, errors: &mut Vec<SemanticError>) -> String {
    let mut result_type = "int".to_string(); // Default type

    for child in &node.children {
        match &child.Symbol {
            Symbol::Terminal(Token::Type(TypeTK::IntVal(_))) => return "int".to_string(),
            Symbol::Terminal(Token::Type(TypeTK::DoubleVal(_))) => return "double".to_string(),
            Symbol::Terminal(Token::Variable(VariableTK::VarName(name))) => {
                if let Some(var_type) = symbol_table.get(name) {
                    if var_type.to_lowercase() == "double" {
                        result_type = "double".to_string(); // Upgrade result to double
                    }
                } else {
                    errors.push(SemanticError {
                        message: format!("Use of undeclared variable '{}'.", name),
                    });
                }
            }
            Symbol::NonTerminal(NonTerminal::ArithEx) => {
                let child_type = evaluate_expression_type(child, symbol_table, errors);
                if child_type == "double" {
                    result_type = "double".to_string(); // Upgrade result to double
                }
            }
            Symbol::NonTerminal(NonTerminal::ArithExP) => {
                let continuation_type = evaluate_arith_ex_p_type(child, symbol_table, errors);
                if continuation_type == "double" {
                    result_type = "double".to_string(); // Upgrade result to double
                }
            }
            _ => {
                let deeper_type = evaluate_expression_type(child, symbol_table, errors);
                if deeper_type == "double" {
                    result_type = "double".to_string(); // Upgrade result to double
                }
            }
        }
    }

    result_type
}

fn evaluate_arith_ex_p_type(node: &TreeNode, symbol_table: &SymbolTable, errors: &mut Vec<SemanticError>) -> String {
    let mut result_type = "int".to_string();
    for child in &node.children {
        match &child.Symbol {
            Symbol::NonTerminal(NonTerminal::ArithVal) => {
                let child_type = evaluate_expression_type(child, symbol_table, errors);
                if child_type == "double" {
                    result_type = "double".to_string();
                }
            }
            Symbol::NonTerminal(NonTerminal::ArithEx) => {
                let continuation_type = evaluate_expression_type(child, symbol_table, errors);
                if continuation_type == "double" {
                    result_type = "double".to_string();
                }
            }
            _ => {}
        }
    }
    result_type
}
