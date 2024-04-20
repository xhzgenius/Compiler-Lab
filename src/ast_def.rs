//! Definition of the Abstract Syntax Tree (AST). 
//! 
//!
//! Currently, AST is defined as follows:
//!
//! CompUnit  ::= FuncDef;
//!
//! FuncDef   ::= FuncType Id "(" ")" Block;
//! FuncType  ::= "int";
//!
//! Block     ::= "{" Stmt "}";
//! Stmt      ::= "return" Number ";";
//! Number    ::= INT_CONST;


#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub return_type: Type,
    pub func_id: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct Type {
    pub type_name: String,
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub stmt: StmtEnum
}

#[derive(Debug)]
pub enum StmtEnum {
    ReturnStmt(Number),
}

#[derive(Debug)]
pub struct Number {
    pub number: NumberEnum
}


#[derive(Debug)]
pub enum NumberEnum {
    IntConst(i32),
}