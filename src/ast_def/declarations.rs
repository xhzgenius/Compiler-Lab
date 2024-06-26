//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of declarations.

use super::expressions::*;
use super::statements::*;
use super::symbols::*;

#[derive(Debug)]
pub enum FuncDef {
    Default(BType, IDENT, Vec<FuncFParam>, Block),
}

#[derive(Debug)]
pub enum FuncFParam {
    Default(BType, IDENT, Option<Vec<Exp>>),
}

#[derive(Debug)]
pub enum Block {
    Default(Vec<BlockItem>), // May have 0 or more items.
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
    Aggregate(Vec<Box<InitVal>>),
}

#[derive(Debug)]
pub enum Decl {
    ConstDecl(ConstDecl),
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub enum ConstDecl {
    Default(BType, Vec<ConstDef>),
}

#[derive(Debug)]
pub enum ConstDef {
    Default(IDENT, Vec<Exp>, InitVal),
}

#[derive(Debug)]
pub enum VarDecl {
    Default(BType, Vec<VarDef>),
}

#[derive(Debug)]
pub enum VarDef {
    Default(IDENT, Vec<Exp>, Option<InitVal>),
}
