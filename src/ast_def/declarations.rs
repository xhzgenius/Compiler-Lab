//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of declarations.

use super::statements::*;
use super::expressions::*;
use super::symbols::*;

#[derive(Debug)]
pub enum FuncDef {
    Default(FuncType, IDENT, Block),
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
    Default(IDENT, ConstInitVal),
}

#[derive(Debug)]
pub enum VarDecl {
    Default(BType, Vec<VarDef>),
}

#[derive(Debug)]
pub enum VarDef {
    WithoutInitVal(IDENT),
    WithInitVal(IDENT, InitVal),
}
