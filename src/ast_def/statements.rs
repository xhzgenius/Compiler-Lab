//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of statements.

use super::declarations::*;
use super::expressions::*;

#[derive(Debug)]
pub enum Stmt {
    AssignStmt(LVal, Exp),
    Exp(Option<Exp>),
    Block(Block),
    ReturnStmt(Exp),
}
