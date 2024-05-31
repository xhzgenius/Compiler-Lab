//! This module is the frontend of my compiler.
//! It converts the C code into Koopa IR.

mod build_declarations;
mod build_expressions;
mod build_statements;
use crate::ast_def::*;
use koopa::ir::builder_traits::BasicBlockBuilder;
use koopa::ir::entities::{BasicBlock, Function, Value}; // Koopa IR builder
use koopa::ir::{Program, Type, TypeKind}; // All the symbol defined in the AST
use std::collections::HashMap;

pub fn generate_ir(comp_unit: &CompUnit) -> Result<Program, String> {
    let mut program = Program::new();
    let mut my_ir_generator_info = MyIRGeneratorInfo {
        curr_block: None,
        break_tgt_blocks: vec![],
        continue_tgt_blocks: vec![],
        curr_func: None,
        symbol_tables: SymbolTableStack {
            symbol_tables: vec![HashMap::new()],
        },
        bb_cnt: 0,
        function_table: HashMap::new(),
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    Ok(program)
}

#[derive(Debug)]
pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>,            // Current block
    break_tgt_blocks: Vec<BasicBlock>,         // Target blocks of break statements
    continue_tgt_blocks: Vec<BasicBlock>,      // Target blocks of continue statements
    curr_func: Option<Function>,               // Current function
    symbol_tables: SymbolTableStack,           // Symbol table: ident-(type, Value)
    bb_cnt: usize,                             // Number of BasicBlocks
    function_table: HashMap<String, Function>, // Function table
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
        // Declare all SysY library functions.
        /*
           decl @getint(): i32
           decl @getch(): i32
           decl @getarray(*i32): i32
           decl @putint(i32)
           decl @putch(i32)
           decl @putarray(i32, *i32)
           decl @starttime()
           decl @stoptime()
        */
        let lib_functions = vec![
            ("getint", vec![], Type::get_i32()),
            ("getch", vec![], Type::get_i32()),
            (
                "getarray",
                vec![Type::get_pointer(Type::get_i32())],
                Type::get_i32(),
            ),
            ("putint", vec![Type::get_i32()], Type::get_unit()),
            ("putch", vec![Type::get_i32()], Type::get_unit()),
            (
                "putarray",
                vec![Type::get_i32(), Type::get_pointer(Type::get_i32())],
                Type::get_unit(),
            ),
            ("starttime", vec![], Type::get_unit()),
            ("stoptime", vec![], Type::get_unit()),
        ];
        for (name, params_ty, ret_ty) in lib_functions {
            let function_data =
                koopa::ir::FunctionData::new_decl(format!("@{}", name), params_ty, ret_ty);
            let func = program.new_func(function_data);
            my_ir_generator_info
                .function_table
                .insert(name.to_string(), func);
        }

        // Build every unit.
        let CompUnit::Default(units) = self;
        for unit in units {
            unit.build(program, my_ir_generator_info)?;
        }
        Ok(IRBuildResult::OK)
    }
}

impl IRBuildable for Unit {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            Unit::Decl(d) => d.build(program, my_ir_generator_info),
            Unit::FuncDef(f) => f.build(program, my_ir_generator_info),
        }
    }
}

/// Helper function to build a new value.
fn create_new_local_value<'a>(
    program: &'a mut Program,
    my_ir_generator_info: &'a mut MyIRGeneratorInfo,
) -> koopa::ir::builder::LocalBuilder<'a> {
    program
        .func_mut(
            my_ir_generator_info
                .curr_func
                .expect("You should use create_new_global_value when curr_func is None! "),
        )
        .dfg_mut()
        .new_value()
}

/// Helper function to build a new basic block.
fn create_new_block<'a>(
    program: &'a mut Program,
    my_ir_generator_info: &'a mut MyIRGeneratorInfo,
    name: &str,
) -> BasicBlock {
    let block = program
        .func_mut(my_ir_generator_info.curr_func.unwrap())
        .dfg_mut()
        .new_bb()
        .basic_block(Some(format!("%bb{}_{}", my_ir_generator_info.bb_cnt, name)));
    my_ir_generator_info.bb_cnt += 1;
    block
}

/// Helper function to insert instructions into the current function's data flow graph.
fn insert_local_instructions<T>(
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
