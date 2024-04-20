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
pub enum Block {
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    ReturnStmt(Exp),
}

#[derive(Debug)]
pub enum Exp {
    LOrExp(LOrExp),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    BinaryLOrExp(Box<LOrExp>, LAndExp),
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(EqExp),
    BinaryLAndExp(Box<LAndExp>, EqExp),
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    BinaryEqExp(Box<EqExp>, RelExp), 
    BinaryUneqExp(Box<EqExp>, RelExp), 
}

#[derive(Debug)]
pub enum RelExp {
    AddExp(AddExp),
    BinaryLtExp(Box<RelExp>, AddExp), 
    BinaryGtExp(Box<RelExp>, AddExp), 
    BinaryLeExp(Box<RelExp>, AddExp), 
    BinaryGeExp(Box<RelExp>, AddExp), 
}

#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    BinaryAddExp(Box<AddExp>, MulExp), 
    BinarySubExp(Box<AddExp>, MulExp), 
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    BinaryMulExp(Box<MulExp>, UnaryExp), 
    BinaryDivExp(Box<MulExp>, UnaryExp), 
    BinaryModExp(Box<MulExp>, UnaryExp), 
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    PlusUnaryExp(Box<UnaryExp>), 
    MinusUnaryExp(Box<UnaryExp>), 
    NotUnaryExp(Box<UnaryExp>), 
}

#[derive(Debug)]
pub enum PrimaryExp {
    BracedExp(Box<Exp>), 
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    IntConst(i32),
}
