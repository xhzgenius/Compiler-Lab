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
        symbol_table: HashMap::new(),
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    Ok(program)
}

#[derive(Debug)]
pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>,                  // Current block
    curr_func: Option<Function>,                     // Current function
    symbol_table: HashMap<String, SymbolTableEntry>, // Symbol table: ident-(type, Value)
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

/// IR building result. If the expression is a constant expression, returns the i32 result. 
/// Otherwise, returns the Koopa IR Value. 
pub enum IRBuildResult {
    Const(i32), 
    Value(Value), 
}

pub trait IRBuildable {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String>;
}

impl IRBuildable for CompUnit {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        self.func_def.build(program, my_ir_generator_info)
    }
}


/// Helper function to build a new value.
fn create_new_value<'a>(
    program: &'a mut Program,
    my_ir_generator_info: &'a mut MyIRGeneratorInfo,
) -> koopa::ir::builder::LocalBuilder<'a> {
    program
        .func_mut(my_ir_generator_info.curr_func.unwrap())
        .dfg_mut()
        .new_value()
}

/// Helper function to insert instructions into the current function's data flow graph.
fn insert_instructions<T>(
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    instructions: T,
) where
    T: IntoIterator<Item = Value>,
{
    program
        .func_mut(my_ir_generator_info.curr_func.unwrap())
        .layout_mut()
        .bb_mut(my_ir_generator_info.curr_block.unwrap())
        .insts_mut()
        .extend(instructions);
}

