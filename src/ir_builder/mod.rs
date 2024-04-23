//! This module is the frontend of my compiler.
//! It converts the C code into Koopa IR.

mod build_declarations;
mod build_expressions;
mod build_statements;
use crate::ast_def::*;
use koopa::ir::entities::{BasicBlock, Function, Value}; // Koopa IR builder
use koopa::ir::{Program, TypeKind}; // All the symbol defined in the AST
use std::collections::HashMap;

pub fn generate_ir(comp_unit: &CompUnit) -> Result<Program, String> {
    let mut program = Program::new();
    let mut my_ir_generator_info = MyIRGeneratorInfo {
        curr_block: None,
        curr_func: None,
        curr_value: None,
        symbol_table: HashMap::new(),
        tmp_constants: None,
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    Ok(program)
}

#[derive(Debug)]
pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>,                  // Current block
    curr_func: Option<Function>,                     // Current function
    curr_value: Option<Value>, // Current value (the target value of operations)
    symbol_table: HashMap<String, SymbolTableEntry>, // Symbol table: ident-(type, Value)
    tmp_constants: Option<(i32, i32)>, // Temporary constant
}

pub enum SymbolTableEntry {
    Variable(TypeKind, Value),
    Constant(TypeKind, Vec<i32>),
}

impl std::fmt::Debug for SymbolTableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolTableEntry::Variable(tk, v) => write!(f, "Variable {}: {:?}", tk, v),
            SymbolTableEntry::Constant(tk, v) => write!(f, "Constant {}: {:?}", tk, v),
        }
    }
}

pub trait IRBuildable {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String>;
}

impl IRBuildable for CompUnit {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        self.func_def.build(program, my_ir_generator_info)?;
        Ok(())
    }
}
