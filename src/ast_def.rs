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
//! Stmt      ::= "return" Exp ";";
//! 
//! Exp         ::= UnaryExp;
//! UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
//! UnaryOp     ::= "+" | "-" | "!";
//! PrimaryExp  ::= "(" Exp ")" | Number;
//! Number      ::= INT_CONST;
//! 


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
pub enum Stmt {
    ReturnStmt(Number),
}

#[derive(Debug)]
pub enum Exp {
    UnaryExp, 
}


#[derive(Debug)]
pub struct UnaryExp {

}
#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}