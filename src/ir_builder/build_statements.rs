//! Build a single component into Koopa IR.

use crate::ast_def::statements::*;
use koopa::ir::{builder_traits::*, Program};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    create_new_value, insert_basic_blocks, insert_instructions, IRBuildResult, IRBuildable,
    MyIRGeneratorInfo,
};

impl IRBuildable for Stmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match &self {
            Stmt::UnmatchedStmt(stmt) => stmt.build(program, my_ir_generator_info),
            Stmt::MatchedStmt(stmt) => stmt.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for UnmatchedStmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        self.default.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for MatchedStmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        self.default.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for BasicStmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match &self {
            BasicStmt::AssignStmt(lval, rhs_exp) => {
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
                // lval_ptr is an alloc instruction. It should not be added to instructions again.
                insert_instructions(program, my_ir_generator_info, [store_inst]);
                Ok(IRBuildResult::OK)
            }
            BasicStmt::Exp(e) => {
                if let Some(exp) = e {
                    exp.build(program, my_ir_generator_info)?;
                    Ok(IRBuildResult::OK)
                } else {
                    Ok(IRBuildResult::OK)
                }
            }
            BasicStmt::Block(block) => block.build(program, my_ir_generator_info),
            BasicStmt::IfStmt(cond, stmt1, possible_stmt2) => {
                let cond_value = match cond.build(program, my_ir_generator_info)? {
                    IRExpBuildResult::Const(int) => {
                        create_new_value(program, my_ir_generator_info).integer(int)
                    }
                    IRExpBuildResult::Value(value) => value,
                };
                let block_end = program
                    .func_mut(my_ir_generator_info.curr_func.unwrap())
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some(format!("%bb{}_if_block_end", my_ir_generator_info.bb_cnt)));
                my_ir_generator_info.bb_cnt += 1;
                let block_start = my_ir_generator_info
                    .curr_block
                    .expect("No current block. Should not happen! ");

                // Build the block 1, which ends with a jump to the ending block.
                let block1 = program
                    .func_mut(my_ir_generator_info.curr_func.unwrap())
                    .dfg_mut()
                    .new_bb()
                    .basic_block(Some(format!("%bb{}_if_block_1", my_ir_generator_info.bb_cnt)));
                my_ir_generator_info.bb_cnt += 1;
                // Remember to insert basic blocks into the current function's data flow graph.
                insert_basic_blocks(program, my_ir_generator_info, [block1]);
                my_ir_generator_info.curr_block = Some(block1);
                match stmt1.build(program, my_ir_generator_info)? {
                    IRBuildResult::OK => {
                        let jmp_inst =
                            create_new_value(program, my_ir_generator_info).jump(block_end);
                        insert_instructions(program, my_ir_generator_info, [jmp_inst]);
                    }
                    IRBuildResult::EARLYSTOPPING => {}
                }

                // If there is block 2, build block 2, which ends with a jump to the ending block.
                let jmp_block = match &**possible_stmt2 {
                    Some(stmt2) => {
                        let block2 = program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_bb()
                            .basic_block(Some(format!("%bb{}_if_block_2", my_ir_generator_info.bb_cnt)));
                        my_ir_generator_info.bb_cnt += 1;
                        // Remember to insert the basic block into the current function's data flow graph.
                        insert_basic_blocks(program, my_ir_generator_info, [block2]);
                        my_ir_generator_info.curr_block = Some(block2);
                        match stmt2.build(program, my_ir_generator_info)? {
                            IRBuildResult::OK => {
                                let jmp_inst =
                                    create_new_value(program, my_ir_generator_info).jump(block_end);
                                insert_instructions(program, my_ir_generator_info, [jmp_inst]);
                            }
                            IRBuildResult::EARLYSTOPPING => {}
                        }
                        block2
                    }
                    None => block_end,
                };
                let if_stmt = create_new_value(program, my_ir_generator_info)
                    .branch(cond_value, block1, jmp_block);
                my_ir_generator_info.curr_block = Some(block_start);
                insert_instructions(program, my_ir_generator_info, [if_stmt]);

                // Continue with the ending block.
                my_ir_generator_info.curr_block = Some(block_end);
                insert_basic_blocks(program, my_ir_generator_info, [block_end]);

                Ok(IRBuildResult::OK)
            }
            BasicStmt::ReturnStmt(returned_exp) => {
                let return_value = match returned_exp {
                    Some(exp) => Some({
                        let result = exp.build(program, my_ir_generator_info)?; // Build the returned Exp into curr_value.
                        match result {
                            IRExpBuildResult::Const(int) => {
                                create_new_value(program, my_ir_generator_info).integer(int)
                            }
                            IRExpBuildResult::Value(value) => value,
                        }
                    }),
                    None => None,
                };
                let return_stmt = create_new_value(program, my_ir_generator_info).ret(return_value);
                insert_instructions(program, my_ir_generator_info, [return_stmt]);
                Ok(IRBuildResult::EARLYSTOPPING)
            }
        }
    }
}
