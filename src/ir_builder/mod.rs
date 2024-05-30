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
        symbol_tables: SymbolTableStack {
            symbol_tables: vec![HashMap::new()],
        },
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    Ok(program)
}

#[derive(Debug)]
pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>,  // Current block
    curr_func: Option<Function>,     // Current function
    symbol_tables: SymbolTableStack, // Symbol table: ident-(type, Value)
}

#[derive(Debug)]
pub struct SymbolTableStack {
    symbol_tables: Vec<HashMap<String, SymbolTableEntry>>, // Symbol table: ident-(type, Value)
}

impl SymbolTableStack {
    fn get(&self, name: &String) -> Option<&SymbolTableEntry> {
        for table in self.symbol_tables.iter().rev() {
            if let Some(symbol) = table.get(name) {
                return Some(symbol);
            }
        }
        None
    }
    // fn get_nearest(&self, name: &String) -> Option<&SymbolTableEntry> {

    // }
    fn insert(&mut self, name: String, entry: SymbolTableEntry) {
        let table = self.symbol_tables.last_mut().unwrap();
        table.insert(name, entry);
    }
    fn add_new_table(&mut self) {
        self.symbol_tables.push(HashMap::new());
    }
    fn delete_new_table(&mut self) {
        self.symbol_tables.pop();
    }
    fn curr_depth(&self) -> usize {
        self.symbol_tables.len() - 1
    }
}

pub enum SymbolTableEntry {
    Variable(TypeKind, Value),
    Constant(TypeKind, Vec<i32>),
}

impl std::fmt::Debug for SymbolTableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolTableEntry::Variable(tk, v) => write!(f, "Variable({}): {:?}", tk, v),
            SymbolTableEntry::Constant(tk, v) => write!(f, "Constant({}): {:?}", tk, v),
        }
    }
}

pub enum IRBuildResult {
    OK,
    EARLYSTOPPING,
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

/// Helper function to insert basic blocks into the current function's data flow graph.
fn insert_basic_blocks<T>(
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    basic_blocks: T,
) where
    T: IntoIterator<Item = BasicBlock>,
{
    program
        .func_mut(my_ir_generator_info.curr_func.unwrap())
        .layout_mut()
        .bbs_mut()
        .extend(basic_blocks);
}
