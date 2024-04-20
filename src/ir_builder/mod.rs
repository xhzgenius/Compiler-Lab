//! This module is the frontend of my compiler.
//! It converts the C code into Koopa IR.

mod build_ir;
use crate::ast_def::*;
use build_ir::IRBuildable;
use koopa::ir::entities::{BasicBlock, Function, Value}; // Koopa IR builder
use koopa::ir::Program; // All the symbol defined in the AST

pub fn generate_ir(comp_unit: &CompUnit) -> Result<Program, String> {
    let mut program = Program::new();
    let mut my_ir_generator_info = MyIRGeneratorInfo {
        curr_block: None, 
        curr_func: None, 
        curr_value: None, 
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    Ok(program)
}

pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>, // Current block
    curr_func: Option<Function>,    // Current function
    curr_value: Option<Value>, // Current value (the target value of operations)
}
