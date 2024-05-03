//! Definitions of the Abstract Syntax Tree (AST).
//! This is the part of expressions.

use super::symbols::*;

#[derive(Debug)]
pub enum ConstInitVal {
    ConstExp(ConstExp),
}

#[derive(Debug)]
pub enum ConstExp {
    Exp(Exp),
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
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
    LVal(LVal),
    Number(Number),
}

#[derive(Debug)]
pub enum LVal {
    IDENT(IDENT),
}

#[derive(Debug)]
pub enum Number {
    INTCONST(i32),
}
