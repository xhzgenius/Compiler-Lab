//! Definition of the Abstract Syntax Tree (AST).
//!
//!
//! Currently, AST is defined as follows:
//!
//! CompUnit ::= {Unit};
//! Unit     ::= Decl | FuncDef;
//!
//! ======================================================
//! Declarations
//! ======================================================
//! FuncDef       ::= FuncType IDENT "(" [FuncFParams] ")" Block;
//! FuncFParams   ::= FuncFParam {"," FuncFParam};
//! FuncFParam    ::= BType IDENT;
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
//! Stmt ::= UnmatchedStmt
//!        | MatchedStmt;
//!
//! UnmatchedStmt ::= "if" "(" Exp ")" MatchedStmt ["else" UnmatchedStmt]
//!                 | "if" "(" Exp ")" UnmatchedStmt
//!                 | "while" "(" Exp ")" UnmatchedStmt;
//!
//! MatchedStmt ::= LVal "=" Exp ";"
//!               | [Exp] ";"
//!               | Block
//!               | "if" "(" Exp ")" MatchedStmt "else" MatchedStmt
//!               | "while" "(" Exp ")" MatchedStmt
//!               | "break" ";"
//!               | "continue" ";"
//!               | "return" [Exp] ";";
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
//! UnaryExp    ::= PrimaryExp
//!               | UnaryOp UnaryExp
//!               | IDENT "(" [FuncRParams] ")";
//! UnaryOp     ::= "+" | "-" | "!";
//! PrimaryExp  ::= "(" Exp ")" | LVal | Number;
//!
//! LVal        ::= IDENT;
//! Number      ::= INTCONST;
//!
//! ======================================================
//! Symbols
//! ======================================================
//! // FuncType  ::= "void" | "int"; (Currently removed to aviod lalrpop conflict. )
//! BType     ::= "int";
//!

pub mod declarations;
pub mod expressions;
pub mod statements;
pub mod symbols;
use declarations::*;

#[derive(Debug)]
pub enum CompUnit {
    Default(Vec<Unit>),
}

#[derive(Debug)]
pub enum Unit {
    Decl(Decl),
    FuncDef(FuncDef),
}
