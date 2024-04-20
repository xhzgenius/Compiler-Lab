//! This module is the backend of my compiler.
//! It converts the Koopa IR into assembly code.

mod build_assembly;
use build_assembly::AssemblyBuildable;
use koopa::ir::{Program, };

pub fn generate_assembly(program: &Program) -> Result<Vec<String>, String> {
    let assembly_codes = program.build()?;
    Ok(assembly_codes)
}