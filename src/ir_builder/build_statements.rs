//! Build a single component into Koopa IR.

use crate::ast_def::statements::*;
use koopa::ir::{builder_traits::*, Program};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    create_new_value, insert_instructions, IRBuildResult, IRBuildable, MyIRGeneratorInfo,
};

impl IRBuildable for Stmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match &self {
            Stmt::AssignStmt(lval, rhs_exp) => {
                // Build LVal value.
                let result1 = lval.build(program, my_ir_generator_info)?;
                let lval_ptr = match result1 {
                    IRExpBuildResult::Const(_int) => {
                        return Err(format!(
                            "Constant expression ({:?}) should not be a left value! ",
                            lval
                        ))
                    }
                    IRExpBuildResult::Value(value) => value,
                };
                // Build RHS value.
                let result2 = rhs_exp.build(program, my_ir_generator_info)?;
                let rhs_value = match result2 {
                    IRExpBuildResult::Const(int) => {
                        create_new_value(program, my_ir_generator_info).integer(int)
                    }
                    IRExpBuildResult::Value(value) => value,
                };
                // Assign the RHS value into the new variable.
                let store_inst =
                    create_new_value(program, my_ir_generator_info).store(rhs_value, lval_ptr);
                insert_instructions(program, my_ir_generator_info, [lval_ptr, store_inst]);
                Ok(IRBuildResult::OK)
            }
            Stmt::ReturnStmt(returned_exp) => {
                let result = returned_exp.build(program, my_ir_generator_info)?; // Build the returned Exp into curr_value.
                let return_value = match result {
                    IRExpBuildResult::Const(int) => {
                        create_new_value(program, my_ir_generator_info).integer(int)
                    }
                    IRExpBuildResult::Value(value) => value,
                };
                let return_stmt =
                    create_new_value(program, my_ir_generator_info).ret(Some(return_value));
                insert_instructions(program, my_ir_generator_info, [return_stmt]);
                Ok(IRBuildResult::EARLYSTOPPING)
            }
            Stmt::Block(block) => block.build(program, my_ir_generator_info),
            Stmt::Exp(e) => {
                if let Some(exp) = e {
                    exp.build(program, my_ir_generator_info)?;
                    Ok(IRBuildResult::OK)
                } else {
                    Ok(IRBuildResult::OK)
                }
            }
        }
    }
}
