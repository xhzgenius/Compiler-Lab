//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of statements.

use super::declarations::*;
use super::expressions::*;

#[derive(Debug)]
pub enum Stmt {
    UnmatchedStmt(UnmatchedStmt),
    MatchedStmt(MatchedStmt),
}

#[derive(Debug)]
pub struct UnmatchedStmt {
    pub default: BasicStmt,
}

#[derive(Debug)]
pub struct MatchedStmt {
    pub default: BasicStmt,
}

#[derive(Debug)]
pub enum BasicStmt {
    AssignStmt(LVal, Exp),
    Exp(Option<Exp>),
    Block(Block),
    IfStmt(Exp, Box<BasicStmt>, Box<Option<BasicStmt>>),
    WhileStmt(Exp, Box<BasicStmt>),
    BreakStmt,
    ContinueStmt,
    ReturnStmt(Option<Exp>),
}
