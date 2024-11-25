#![allow(warnings)]

use crate::lexer::{self, BinaryOpsTK, ControlFlowTK, OpsTK, ScopeTK, Token, TypeTK, UtilitiesTK, VariableTK};
use std::{collections::{hash_map, HashMap}, ops::Index};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonTerminal {
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
    StringP,
    // FnCall,
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
pub enum Symbol {
    Terminal(Token),
    NonTerminal(NonTerminal),
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub children: Vec<TreeNode>,
    pub(crate) Symbol: Symbol,
}

impl TreeNode {
    pub fn debug_print(&self, indent: usize) {
        println!("{}{:?}", " ".repeat(indent), self.Symbol);
        for child in &self.children {
            child.debug_print(indent + 2);
        }
    }
}

fn build_parse_table() -> HashMap<(NonTerminal, Token), Production> {
    let mut table: HashMap<(NonTerminal, Token), Production> = HashMap::new();

    //Prog
    table.insert((NonTerminal::Prog, Token::EOF), Production::Epsilon);
    table.insert((NonTerminal::Prog, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::FuncList)]));
    //FuncList
    table.insert((NonTerminal::FuncList, Token::EOF), Production::Epsilon);
    table.insert((NonTerminal::FuncList, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::FuncDecl), Symbol::NonTerminal(NonTerminal::FuncList)]));
    table.insert((NonTerminal::FuncList, Token::Type(TypeTK::Double)),Production::Rule(vec![Symbol::NonTerminal(NonTerminal::FuncDecl), Symbol::NonTerminal(NonTerminal::FuncList)]));
    // FuncDecl
    table.insert((NonTerminal::FuncDecl, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ListOfParams), Symbol::Terminal(Token::Scope(ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(ScopeTK::CurlyBracketL)), Symbol::NonTerminal(NonTerminal::StatementList), Symbol::Terminal(Token::Scope(ScopeTK::CurlyBracketR))]));
    table.insert((NonTerminal::FuncDecl, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ListOfParams), Symbol::Terminal(Token::Scope(ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(ScopeTK::CurlyBracketL)), Symbol::NonTerminal(NonTerminal::StatementList), Symbol::Terminal(Token::Scope(ScopeTK::CurlyBracketR))]));
    // ListOfParams
    table.insert((NonTerminal::ListOfParams, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::ListOfParams, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::NonEmptyListOfParams)]));
    table.insert((NonTerminal::ListOfParams, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::NonEmptyListOfParams)]));
    // NonEmptyListOfParams
    table.insert((NonTerminal::NonEmptyListOfParams, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::NonTerminal(NonTerminal::NonEmptyListOfParamsContinue)]));
    table.insert((NonTerminal::NonEmptyListOfParams, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::NonTerminal(NonTerminal::NonEmptyListOfParamsContinue)]));
    // NonEmptyListOfParamsContinue
    table.insert((NonTerminal::NonEmptyListOfParamsContinue, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::NonEmptyListOfParamsContinue, Token::Scope(ScopeTK::Comma)), Production::Rule(vec![Symbol::Terminal(Token::Scope(lexer::ScopeTK::Comma)), Symbol::NonTerminal(NonTerminal::NonEmptyListOfParamsContinue)]));
    // StatementList
    table.insert((NonTerminal::StatementList, Token::Scope(ScopeTK::CurlyBracketR)), Production::Epsilon);
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::If)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::While)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::For)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::Break)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::Continue)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::ControlFlow(ControlFlowTK::Return)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::Utilities(UtilitiesTK::Print)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::Type(TypeTK::Const)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    table.insert((NonTerminal::StatementList, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Statement), Symbol::NonTerminal(NonTerminal::StatementList)]));
    // Statement
    table.insert((NonTerminal::Statement, Token::Scope(ScopeTK::CurlyBracketR)), Production::Epsilon);
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::If)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::If)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::BoolEx), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketL)),Symbol::NonTerminal(NonTerminal::StatementList), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketR))]));
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::While)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::While)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::BoolEx), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketL)),Symbol::NonTerminal(NonTerminal::StatementList), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketR))]));
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::For)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::For)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ForLoopFirstBit), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi)), Symbol::NonTerminal(NonTerminal::ForLoopMiddleBit), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi)), Symbol::NonTerminal(NonTerminal::ForLoopLastBit), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketL)),Symbol::NonTerminal(NonTerminal::StatementList), Symbol::Terminal(Token::Scope(lexer::ScopeTK::CurlyBracketR))]));
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::Break)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::Break)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi))]));
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::Continue)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::Continue)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi))]));
    table.insert((NonTerminal::Statement, Token::ControlFlow(ControlFlowTK::Return)), Production::Rule(vec![Symbol::Terminal(Token::ControlFlow(lexer::ControlFlowTK::Return)),Symbol::NonTerminal(NonTerminal::ReturnTail), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi))]));
    table.insert((NonTerminal::Statement, Token::Utilities(UtilitiesTK::Print)), Production::Rule(vec![Symbol::Terminal(Token::Utilities(UtilitiesTK::Print)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::Text), Symbol::Terminal(Token::Scope(lexer::ScopeTK::BracketR)), Symbol::Terminal(Token::Scope(lexer::ScopeTK::Semi))]));
    table.insert((NonTerminal::Statement, Token::Type(TypeTK::Const)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::Statement, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::Statement, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::Statement, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Assignment)]));
    // ForLoopFirstBit
    table.insert((NonTerminal::ForLoopFirstBit, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::ForLoopFirstBit, Token::Type(TypeTK::Const)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::ForLoopFirstBit, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::ForLoopFirstBit, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VarDecl)]));
    table.insert((NonTerminal::ForLoopFirstBit, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Assignment)]));
    // ForLoopMiddleBit
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::ForLoopMiddleBit, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    // ForLoopLastBit
    table.insert((NonTerminal::ForLoopLastBit, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::ForLoopLastBit, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Assignment)]));
    // ReturnTail
    table.insert((NonTerminal::ReturnTail, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::String)]));
    table.insert((NonTerminal::ReturnTail, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    table.insert((NonTerminal::ReturnTail, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    // Text
    table.insert((NonTerminal::Text, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::Text, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::TextElement), Symbol::NonTerminal(NonTerminal::TextTail)]));
    table.insert((NonTerminal::Text, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::TextElement), Symbol::NonTerminal(NonTerminal::TextTail)]));
    table.insert((NonTerminal::Text, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::TextElement), Symbol::NonTerminal(NonTerminal::TextTail)]));
    // TextElement
    table.insert((NonTerminal::TextElement, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::String)]));
    table.insert((NonTerminal::TextElement, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    table.insert((NonTerminal::TextElement, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));   
    // TextTail
    table.insert((NonTerminal::TextTail, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::TextTail, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::String)]));
    table.insert((NonTerminal::TextTail, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    table.insert((NonTerminal::TextTail, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));       
    // Assignment
    table.insert((NonTerminal::Assignment, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VName), Symbol::Terminal(Token::Ops(OpsTK::Assignment)), Symbol::NonTerminal(NonTerminal::Ex), Symbol::Terminal(Token::Scope(ScopeTK::Semi))]));
    // VarDecl
    table.insert((NonTerminal::VarDecl, Token::Type(TypeTK::Const)), Production::Rule(vec![Symbol::Terminal(Token::Type(TypeTK::Const)), Symbol::NonTerminal(NonTerminal::Decl), Symbol::Terminal(Token::Ops(OpsTK::Assignment)), Symbol::NonTerminal(NonTerminal::Ex), Symbol::Terminal(Token::Scope(ScopeTK::Semi))]));
    table.insert((NonTerminal::VarDecl, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::NonTerminal(NonTerminal::VarDeclP), Symbol::Terminal(Token::Scope(ScopeTK::Semi))]));
    table.insert((NonTerminal::VarDecl, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Decl), Symbol::NonTerminal(NonTerminal::VarDeclP), Symbol::Terminal(Token::Scope(ScopeTK::Semi))]));
    // VarDeclP
    table.insert((NonTerminal::VarDeclP, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::VarDeclP, Token::Ops(OpsTK::Assignment)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Assignment)), Symbol::NonTerminal(NonTerminal::Ex)]));
    // Decl
    table.insert((NonTerminal::Decl, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Type), Symbol::NonTerminal(NonTerminal::VName)]));
    table.insert((NonTerminal::Decl, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Type), Symbol::NonTerminal(NonTerminal::VName)]));
    // Ex
    table.insert((NonTerminal::Ex, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::Ex, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolEx)]));
    // BoolEx
    table.insert((NonTerminal::BoolEx, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    table.insert((NonTerminal::BoolEx, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelEx), Symbol::NonTerminal(NonTerminal::BoolExP)]));
    // BoolExP
    table.insert((NonTerminal::BoolExP, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::BoolExP, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::BoolExP, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolOp), Symbol::NonTerminal(NonTerminal::BoolEx)]));
    table.insert((NonTerminal::BoolExP, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::BoolOp), Symbol::NonTerminal(NonTerminal::BoolEx)]));
    // BoolOp
    table.insert((NonTerminal::BoolOp, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::And))]));
    table.insert((NonTerminal::BoolOp, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::Or))]));
    // RelEx
    table.insert((NonTerminal::RelEx, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Scope(ScopeTK::BracketL)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    table.insert((NonTerminal::RelEx, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithEx), Symbol::NonTerminal(NonTerminal::RelExP)]));
    // RelExP
    table.insert((NonTerminal::RelExP, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::And)), Production::Epsilon);
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::Or)), Production::Epsilon);
    table.insert((NonTerminal::RelExP, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::RelExP, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::RelOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    // RelOp
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::Equal))]));
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::LessThan))]));
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::LessThanEqual))]));
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::GreaterThan))]));
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::GreaterThanEqual))]));
    table.insert((NonTerminal::RelOp, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::Terminal(Token::BinaryOps(BinaryOpsTK::NotEqual))]));
    // ArithEx
    table.insert((NonTerminal::ArithEx, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Scope(ScopeTK::BracketL)), Production::Rule(vec![Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ArithEx),Symbol::Terminal(Token::Scope(ScopeTK::BracketR))]));
    table.insert((NonTerminal::ArithEx, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::Terminal(Token::Utilities(UtilitiesTK::ToINT)), Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ArithEx),Symbol::Terminal(Token::Scope(ScopeTK::BracketR))]));
    table.insert((NonTerminal::ArithEx, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::Terminal(Token::Utilities(UtilitiesTK::ToDouble)), Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ArithEx),Symbol::Terminal(Token::Scope(ScopeTK::BracketR))]));
    table.insert((NonTerminal::ArithEx, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    table.insert((NonTerminal::ArithEx, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithVal), Symbol::NonTerminal(NonTerminal::ArithExP)]));
    // ArithExP
    table.insert((NonTerminal::ArithExP, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::ArithExP, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);    
    table.insert((NonTerminal::ArithExP, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::ArithExP, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::ArithExP, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::ArithExP, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    table.insert((NonTerminal::ArithExP, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOp), Symbol::NonTerminal(NonTerminal::ArithEx)]));
    // ArithOp
    table.insert((NonTerminal::ArithOp, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Plus))]));
    table.insert((NonTerminal::ArithOp, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Minus))]));
    table.insert((NonTerminal::ArithOp, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOpP)]));
    table.insert((NonTerminal::ArithOp, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOpP)]));
    table.insert((NonTerminal::ArithOp, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::ArithOpP)]));
    // ArithOpP
    table.insert((NonTerminal::ArithOpP, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Times))]));
    table.insert((NonTerminal::ArithOpP, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Divide))]));
    table.insert((NonTerminal::ArithOpP, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::Terminal(Token::Ops(OpsTK::Modulo))]));
    // ArithVal
    table.insert((NonTerminal::ArithVal, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::String)]));
    table.insert((NonTerminal::ArithVal, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    table.insert((NonTerminal::ArithVal, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Number)]));
    // String
    table.insert((NonTerminal::String, Token::Variable(VariableTK::VarName(String::new()))),  Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VName), Symbol::NonTerminal(NonTerminal::StringP)]));
    //StringP
    table.insert((NonTerminal::StringP, Token::Type(TypeTK::IntVal(0))),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Scope(ScopeTK::Semi)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Ops(OpsTK::Plus)),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Ops(OpsTK::Minus)),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Ops(OpsTK::Times)),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Ops(OpsTK::Divide)),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Ops(OpsTK::Modulo)),  Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Epsilon);
    table.insert((NonTerminal::StringP, Token::Scope(ScopeTK::BracketL)), Production::Rule(vec![Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ArgList), Symbol::Terminal(Token::Scope(ScopeTK::BracketR))]));
    // FnCall
    // table.insert((NonTerminal::FnCall, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::VName), Symbol::Terminal(Token::Scope(ScopeTK::BracketL)), Symbol::NonTerminal(NonTerminal::ArgList),Symbol::Terminal(Token::Scope(ScopeTK::BracketR))]));
    // ArgList
    table.insert((NonTerminal::ArgList, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::ArgList, Token::Ops(OpsTK::Plus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Ops(OpsTK::Minus)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Ops(OpsTK::Times)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Ops(OpsTK::Divide)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Ops(OpsTK::Modulo)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::And)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::Or)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::Equal)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::LessThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::LessThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::GreaterThan)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::GreaterThanEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::BinaryOps(BinaryOpsTK::NotEqual)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Utilities(UtilitiesTK::ToINT)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Utilities(UtilitiesTK::ToDouble)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    table.insert((NonTerminal::ArgList, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    // ArgListTail
    table.insert((NonTerminal::ArgListTail, Token::Scope(ScopeTK::BracketR)), Production::Epsilon);
    table.insert((NonTerminal::ArgListTail, Token::Scope(ScopeTK::Comma)), Production::Rule(vec![Symbol::NonTerminal(NonTerminal::Ex), Symbol::NonTerminal(NonTerminal::ArgListTail)]));
    // Type
    table.insert((NonTerminal::Type, Token::Type(TypeTK::Int)), Production::Rule(vec![Symbol::Terminal(Token::Type(TypeTK::Int))]));
    table.insert((NonTerminal::Type, Token::Type(TypeTK::Double)), Production::Rule(vec![Symbol::Terminal(Token::Type(TypeTK::Double))]));
    // VName
    table.insert((NonTerminal::VName, Token::Variable(VariableTK::VarName(String::new()))), Production::Rule(vec![Symbol::Terminal(Token::Variable(VariableTK::VarName(String::new())))])); 
    // Number
    table.insert((NonTerminal::Number, Token::Type(TypeTK::IntVal(0))), Production::Rule(vec![Symbol::Terminal(Token::Type(TypeTK::IntVal(0)))]));
    table.insert((NonTerminal::Number, Token::Type(TypeTK::DoubleVal(String::new()))), Production::Rule(vec![Symbol::Terminal(Token::Type(TypeTK::DoubleVal(String::new())))]));

    table
} 

pub fn parser(tokens: &[Token]) -> Result<TreeNode, String> {
    let table = build_parse_table();
    let mut index= 0;

    let root = parse_non_terminal(NonTerminal::Prog, tokens, &mut index, &table)?;

    if index < tokens.len() && !matches!(tokens[index], Token::EOF) {
        return Err(format!(
            "Unexpected token at the end: {:?}",
            tokens.get(index)
        ));
    }

    Ok(root)
}

//if the token is a value to get past the parse table having issues with it not being the exact same value it replaces the token temperately with the one it expects
fn token_stuffer(current_token: Token) -> Token{
    if matches!(current_token, Token::Type(TypeTK::IntVal(_))) {
         return Token::Type(TypeTK::IntVal(0));
    }

    if matches!(current_token, Token::Type(TypeTK::DoubleVal(_))) {
        return Token::Type(TypeTK::DoubleVal(String::new()));
    }

    if matches!(current_token, Token::Variable(VariableTK::VarName(_))) {
        return Token::Variable(VariableTK::VarName(String::new()));
    }

    current_token.clone()
}

fn parse_non_terminal(non_terminal: NonTerminal, tokens: &[Token], index: &mut usize, table: &HashMap<(NonTerminal, Token), Production>) -> Result<TreeNode, String> {
    let mut node = TreeNode{
        children: vec![],
        Symbol: Symbol::NonTerminal(non_terminal.clone()),
    };

    let current_token = if *index < tokens.len() {
        &tokens[*index]
    } else {
        return  Err("Unexpected end of input.".to_string());
    };

    println!("Parsing non-terminal: {:?}", non_terminal);
    println!("Current token: {:?}", tokens[*index].clone());
    println!();
        
    if let Some(production) = table.get(&(non_terminal.clone(), token_stuffer(tokens[*index].clone()))) {
        match production {
            Production::Rule(symbols) => {
                for symbol in symbols {
                    match symbol {
                        Symbol::Terminal(_expected_token) => {
                            node.children.push(TreeNode {
                                children: vec![],
                                Symbol: Symbol::Terminal(tokens[*index].clone()), //for some reason current token is different IDK weird
                            });
                            *index += 1; 
                        }
                        Symbol::NonTerminal(next_non_terminal) => {
                            node.children.push(parse_non_terminal(next_non_terminal.clone(), tokens, index, table)?);
                        }
                    }
                }
            }
            Production::Epsilon => {
                //do nothing
            }
        }
    } else {
        return Err(format!(
            "No production rule for non-terminal {:?} with token {:?}",
            non_terminal, current_token
        ));
    }

    Ok(node)
}