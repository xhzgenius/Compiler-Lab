//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::{MyIRGeneratorInfo, IRBuildable};

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            FuncDef::Default(return_type, func_id, block) => {
                let return_type = match return_type.content.as_str() {
                    "int" => Ok(Type::get_i32()),
                    _ => Err("Wrong return type"),
                };
                // dbg!("Building function", &self);
                let func = program.new_func(FunctionData::with_param_names(
                    "@".to_string() + func_id.content.as_str(),
                    vec![],
                    return_type.expect("Error creating new function"),
                ));
                let func_data = program.func_mut(func);
                let new_block = func_data
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some("%entry".to_string()));
                func_data.layout_mut().bbs_mut().extend([new_block]);
                my_ir_generator_info.curr_block = Some(new_block);
                my_ir_generator_info.curr_func = Some(func);
                block.build(program, my_ir_generator_info)?;
                Ok(())
            }
        }
    }
}

impl IRBuildable for Block {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Block::Default(stmts) => {
                for stmt in stmts {
                    stmt.build(program, my_ir_generator_info)?
                }
                Ok(())
            }
        }
    }
}

impl IRBuildable for BlockItem {
    fn build(
            &self,
            program: &mut Program,
            my_ir_generator_info: &mut MyIRGeneratorInfo,
        ) -> Result<(), String> {
        todo!()
    }
}