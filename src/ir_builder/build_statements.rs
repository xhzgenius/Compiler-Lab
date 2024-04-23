//! Build a single component into Koopa IR.

use crate::ast_def::statements::*;
use koopa::ir::{builder_traits::*, Program};

use super::{IRBuildable, MyIRGeneratorInfo};

impl IRBuildable for Stmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match &self {
            Stmt::AssignStmt(lval, rhs_exp) => {
                // Build LVal value.
                lval.build(program, my_ir_generator_info)?;
                let lval_ptr = my_ir_generator_info.curr_value.unwrap();
                // Build RHS value.
                rhs_exp.build(program, my_ir_generator_info)?;
                let rhs_value = my_ir_generator_info.curr_value.unwrap();
                // Assign the RHS value into the new variable.
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                let dfg = curr_func_data.dfg_mut();
                let store_inst = dfg.new_value().store(rhs_value, lval_ptr);
                curr_func_data
                    .layout_mut()
                    .bb_mut(my_ir_generator_info.curr_block.unwrap())
                    .insts_mut()
                    .extend([lval_ptr, store_inst]);
                Ok(())
            }
            Stmt::ReturnStmt(returned_exp) => {
                returned_exp.build(program, my_ir_generator_info)?; // Build the returned Exp into curr_value.
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                let return_stmt = curr_func_data
                    .dfg_mut()
                    .new_value()
                    .ret(my_ir_generator_info.curr_value); // Could not be None!
                curr_func_data
                    .layout_mut()
                    .bb_mut(my_ir_generator_info.curr_block.unwrap())
                    .insts_mut()
                    .extend([return_stmt]);
                Ok(())
            }
        }
    }
}
