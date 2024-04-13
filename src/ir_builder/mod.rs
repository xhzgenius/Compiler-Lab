//! This module is the frontend of my compiler.
//! It converts the C code into Koopa IR.

mod build;
use crate::ast_def::*;
use build::Buildable;
use koopa::ir::entities::{BasicBlock, Function}; // Koopa IR builder
use koopa::ir::Program; // All the symbol defined in the AST

pub fn generate_ir(comp_unit: &CompUnit) -> Result<Program, &str> {
    let mut program = Program::new();
    let mut my_ir_generator_info = MyIRGeneratorInfo {
        curr_block: None,
        curr_func: None,
    };
    comp_unit.build(&mut program, &mut my_ir_generator_info)?;
    return Ok(program);
}

pub struct MyIRGeneratorInfo {
    curr_block: Option<BasicBlock>, // Current block
    curr_func: Option<Function>,    // Current function
}
