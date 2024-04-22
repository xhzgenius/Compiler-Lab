//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of statements.

use super::expressions::*;

#[derive(Debug)]
pub enum Stmt {
    ReturnStmt(Exp),
    AssignStmt(LVal, Exp),
}
