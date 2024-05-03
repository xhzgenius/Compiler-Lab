//! Definition of the Abstract Syntax Tree (AST).
//!
//!
//! Currently, AST is defined as follows:
//!
//! CompUnit  ::= FuncDef;
//!
//! ======================================================
//! Declarations
//! ======================================================
//! FuncDef   ::= FuncType IDENT "(" ")" Block;
//!
//! Block         ::= "{" {BlockItem} "}";
//! BlockItem     ::= Decl | Stmt;
//!
//! Decl          ::= ConstDecl | VarDecl;
//!
//! ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
//! ConstDef      ::= IDENT "=" ConstInitVal;
//!
//! VarDecl       ::= BType VarDef {"," VarDef} ";";
//! VarDef        ::= IDENT | IDENT "=" InitVal;
//!
//! ======================================================
//! Statements
//! ======================================================
//! Stmt ::= LVal "=" Exp ";"
//!        | [Exp] ";"
//!        | Block
//!        | "return" [Exp] ";";
//!
//! ======================================================
//! Expressions
//! ======================================================
//! ConstInitVal  ::= ConstExp;
//! ConstExp      ::= Exp;
//! InitVal       ::= Exp;
//! 
//! Exp         ::= LOrExp;
//!
//! LOrExp      ::= LAndExp | LOrExp "||" LAndExp;
//! LAndExp     ::= EqExp | LAndExp "&&" EqExp;
//! EqExp       ::= RelExp | EqExp ("==" | "!=") RelExp;
//! RelExp      ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
//!
//! AddExp      ::= MulExp | AddExp ("+" | "-") MulExp;
//! MulExp      ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
//!
//! UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
//! UnaryOp     ::= "+" | "-" | "!";
//! PrimaryExp    ::= "(" Exp ")" | LVal | Number;
//!
//! LVal        ::= IDENT;
//! Number      ::= INTCONST;
//!
//! ======================================================
//! Symbols
//! ======================================================
//! FuncType  ::= "int";
//! BType     ::= "int";
//! 

pub mod declarations;
pub mod statements;
pub mod expressions;
pub mod symbols;
use declarations::*;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}
