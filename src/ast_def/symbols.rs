use std::fmt::Debug;

use koopa::ir::TypeKind;

#[derive(Debug)]
pub struct IDENT {
    pub content: String,
}

pub struct FuncType {
    pub content: TypeKind,
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub struct BType {
    pub content: TypeKind,
}

impl Debug for BType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}