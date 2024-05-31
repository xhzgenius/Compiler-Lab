//! Build a single component into Koopa IR.

use crate::ast_def::expressions::*;
use koopa::ir::{builder_traits::*, Program, Type, Value};

use super::{
    create_new_value, insert_basic_blocks, insert_instructions, MyIRGeneratorInfo, SymbolTableEntry,
};

/// IR expression building result. If the expression is a constant expression, returns the i32 result.
/// Otherwise, returns the Koopa IR Value.
pub enum IRExpBuildResult {
    Const(i32),
    Value(Value),
}
pub trait IRExpBuildable {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String>;
}

impl IRExpBuildable for ConstInitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        let ConstInitVal::ConstExp(c) = self;
        c.build(program, my_ir_generator_info)
    }
}

impl IRExpBuildable for ConstExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        let ConstExp::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)
    }
}

impl IRExpBuildable for InitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        let InitVal::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)
    }
}

impl IRExpBuildable for Exp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            Exp::LOrExp(lor_exp) => lor_exp.build(program, my_ir_generator_info),
        }
    }
}

fn build_binary_from_build_results(
    result1: IRExpBuildResult,
    result2: IRExpBuildResult,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<IRExpBuildResult, String> {
    // If both expressions are constant expressions, then the result should be a constant expression.
    if let (IRExpBuildResult::Const(int1), IRExpBuildResult::Const(int2)) = (&result1, &result2) {
        Ok(IRExpBuildResult::Const(match binary_op {
            koopa::ir::BinaryOp::NotEq => (int1 != int2) as i32,
            koopa::ir::BinaryOp::Eq => (int1 == int2) as i32,
            koopa::ir::BinaryOp::Gt => (int1 > int2) as i32,
            koopa::ir::BinaryOp::Lt => (int1 < int2) as i32,
            koopa::ir::BinaryOp::Ge => (int1 >= int2) as i32,
            koopa::ir::BinaryOp::Le => (int1 <= int2) as i32,
            koopa::ir::BinaryOp::Add => int1 + int2,
            koopa::ir::BinaryOp::Sub => int1 - int2,
            koopa::ir::BinaryOp::Mul => int1 * int2,
            koopa::ir::BinaryOp::Div => int1 / int2,
            koopa::ir::BinaryOp::Mod => int1 % int2,
            koopa::ir::BinaryOp::And => int1 & int2,
            koopa::ir::BinaryOp::Or => int1 | int2,
            koopa::ir::BinaryOp::Xor => int1 ^ int2,
            koopa::ir::BinaryOp::Shl => todo!(),
            koopa::ir::BinaryOp::Shr => todo!(),
            koopa::ir::BinaryOp::Sar => todo!(),
        }))
    } else {
        let value1 = match result1 {
            IRExpBuildResult::Const(int) => {
                create_new_value(program, my_ir_generator_info).integer(int)
            }
            IRExpBuildResult::Value(value) => value,
        };
        let value2 = match result2 {
            IRExpBuildResult::Const(int) => {
                create_new_value(program, my_ir_generator_info).integer(int)
            }
            IRExpBuildResult::Value(value) => value,
        };
        let new_value =
            create_new_value(program, my_ir_generator_info).binary(binary_op, value1, value2);
        insert_instructions(program, my_ir_generator_info, [new_value]);
        Ok(IRExpBuildResult::Value(new_value))
    }
}

impl IRExpBuildable for LOrExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            LOrExp::LAndExp(exp) => exp.build(program, my_ir_generator_info),
            LOrExp::BinaryLOrExp(exp1, exp2) => {
                // Build exp1.
                let exp1_build_result = exp1.build(program, my_ir_generator_info)?;

                match exp1_build_result {
                    // If exp1 is variable.
                    IRExpBuildResult::Value(value1) => {
                        // Prepare for shortcut.
                        /*
                           int result = 1;
                           if (lhs == 0) {
                               result = rhs!=0;
                           }
                        */
                        let block1 = program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_bb()
                            .basic_block(Some(format!("%bb{}_LOr_if_block_1", my_ir_generator_info.bb_cnt)));
                        my_ir_generator_info.bb_cnt += 1;
                        let block_end = program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_bb()
                            .basic_block(Some(format!("%bb{}_LOr_if_block_end", my_ir_generator_info.bb_cnt)));
                        my_ir_generator_info.bb_cnt += 1;
                        insert_basic_blocks(program, my_ir_generator_info, [block1, block_end]);

                        let result_ptr =
                            create_new_value(program, my_ir_generator_info).alloc(Type::get_i32());
                        program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .set_value_name(result_ptr, Some(format!("@LOr_result")));
                        let one = create_new_value(program, my_ir_generator_info).integer(1);
                        let zero = create_new_value(program, my_ir_generator_info).integer(0);
                        let store_inst =
                            create_new_value(program, my_ir_generator_info).store(one, result_ptr);

                        let cond = create_new_value(program, my_ir_generator_info).binary(
                            koopa::ir::BinaryOp::Eq,
                            value1,
                            zero,
                        );
                        let branch_inst = create_new_value(program, my_ir_generator_info)
                            .branch(cond, block1, block_end);
                        insert_instructions(
                            program,
                            my_ir_generator_info,
                            [result_ptr, store_inst, cond, branch_inst],
                        );

                        my_ir_generator_info.curr_block = Some(block1);
                        let result2 = build_binary_from_build_results(
                            IRExpBuildResult::Const(0),
                            exp2.build(program, my_ir_generator_info)?,
                            program,
                            my_ir_generator_info,
                            koopa::ir::BinaryOp::NotEq,
                        )?;
                        let value2 = match result2 {
                            IRExpBuildResult::Const(i2) => {
                                create_new_value(program, my_ir_generator_info).integer(i2)
                            }
                            IRExpBuildResult::Value(v2) => v2,
                        };
                        let store_new_inst = create_new_value(program, my_ir_generator_info)
                            .store(value2, result_ptr);
                        let jmp_inst =
                            create_new_value(program, my_ir_generator_info).jump(block_end);
                        insert_instructions(
                            program,
                            my_ir_generator_info,
                            [store_new_inst, jmp_inst],
                        );

                        my_ir_generator_info.curr_block = Some(block_end);
                        let loaded_result =
                            create_new_value(program, my_ir_generator_info).load(result_ptr);
                        insert_instructions(program, my_ir_generator_info, [loaded_result]);
                        Ok(IRExpBuildResult::Value(loaded_result))
                    }

                    // If exp1 is constant.
                    IRExpBuildResult::Const(i1) => {
                        if i1 != 0 {
                            Ok(IRExpBuildResult::Const(1))
                        } else {
                            build_binary_from_build_results(
                                IRExpBuildResult::Const(0),
                                exp2.build(program, my_ir_generator_info)?,
                                program,
                                my_ir_generator_info,
                                koopa::ir::BinaryOp::NotEq,
                            )
                        }
                    }
                }
            }
        }
    }
}

impl IRExpBuildable for LAndExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            LAndExp::EqExp(exp) => exp.build(program, my_ir_generator_info),
            LAndExp::BinaryLAndExp(exp1, exp2) => {
                // Build exp1.
                let exp1_build_result = exp1.build(program, my_ir_generator_info)?;

                match exp1_build_result {
                    // If exp1 is variable.
                    IRExpBuildResult::Value(value1) => {
                        // Prepare for shortcut.
                        /*
                           int result = 0;
                           if (lhs != 0) {
                               result = rhs!=0;
                           }
                        */
                        let block1 = program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_bb()
                            .basic_block(Some(format!("%bb{}_LAnd_if_block_1", my_ir_generator_info.bb_cnt)));
                        my_ir_generator_info.bb_cnt += 1;
                        let block_end = program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_bb()
                            .basic_block(Some(format!("%bb{}_LAnd_if_block_end", my_ir_generator_info.bb_cnt)));
                        my_ir_generator_info.bb_cnt += 1;
                        insert_basic_blocks(program, my_ir_generator_info, [block1, block_end]);

                        let result_ptr =
                            create_new_value(program, my_ir_generator_info).alloc(Type::get_i32());
                        program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .set_value_name(result_ptr, Some(format!("@LAnd_result")));
                        let zero = create_new_value(program, my_ir_generator_info).integer(0);
                        let store_inst =
                            create_new_value(program, my_ir_generator_info).store(zero, result_ptr);

                        let cond = create_new_value(program, my_ir_generator_info).binary(
                            koopa::ir::BinaryOp::NotEq,
                            value1,
                            zero,
                        );
                        let branch_inst = create_new_value(program, my_ir_generator_info)
                            .branch(cond, block1, block_end);
                        insert_instructions(
                            program,
                            my_ir_generator_info,
                            [result_ptr, store_inst, cond, branch_inst],
                        );

                        my_ir_generator_info.curr_block = Some(block1);
                        let result2 = build_binary_from_build_results(
                            IRExpBuildResult::Const(0),
                            exp2.build(program, my_ir_generator_info)?,
                            program,
                            my_ir_generator_info,
                            koopa::ir::BinaryOp::NotEq,
                        )?;
                        let value2 = match result2 {
                            IRExpBuildResult::Const(i2) => {
                                create_new_value(program, my_ir_generator_info).integer(i2)
                            }
                            IRExpBuildResult::Value(v2) => v2,
                        };
                        let store_new_inst = create_new_value(program, my_ir_generator_info)
                            .store(value2, result_ptr);
                        let jmp_inst =
                            create_new_value(program, my_ir_generator_info).jump(block_end);
                        insert_instructions(
                            program,
                            my_ir_generator_info,
                            [store_new_inst, jmp_inst],
                        );

                        my_ir_generator_info.curr_block = Some(block_end);
                        let loaded_result =
                            create_new_value(program, my_ir_generator_info).load(result_ptr);
                        insert_instructions(program, my_ir_generator_info, [loaded_result]);
                        Ok(IRExpBuildResult::Value(loaded_result))
                    }

                    // If exp1 is constant.
                    IRExpBuildResult::Const(i1) => {
                        if i1 == 0 {
                            Ok(IRExpBuildResult::Const(0))
                        } else {
                            build_binary_from_build_results(
                                IRExpBuildResult::Const(0),
                                exp2.build(program, my_ir_generator_info)?,
                                program,
                                my_ir_generator_info,
                                koopa::ir::BinaryOp::NotEq,
                            )
                        }
                    }
                }
            }
        }
    }
}

impl IRExpBuildable for EqExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            EqExp::RelExp(exp) => exp.build(program, my_ir_generator_info),
            EqExp::BinaryEqExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Eq,
            ),
            EqExp::BinaryUneqExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::NotEq,
            ),
        }
    }
}

impl IRExpBuildable for RelExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            RelExp::AddExp(exp) => exp.build(program, my_ir_generator_info),
            RelExp::BinaryLtExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Lt,
            ),
            RelExp::BinaryGtExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Gt,
            ),
            RelExp::BinaryLeExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Le,
            ),
            RelExp::BinaryGeExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Ge,
            ),
        }
    }
}

impl IRExpBuildable for AddExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            AddExp::MulExp(exp) => exp.build(program, my_ir_generator_info),
            AddExp::BinaryAddExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Add,
            ),
            AddExp::BinarySubExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
        }
    }
}

impl IRExpBuildable for MulExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            MulExp::UnaryExp(exp) => exp.build(program, my_ir_generator_info),
            MulExp::BinaryMulExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Mul,
            ),
            MulExp::BinaryDivExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Div,
            ),
            MulExp::BinaryModExp(exp1, exp2) => build_binary_from_build_results(
                exp1.build(program, my_ir_generator_info)?,
                exp2.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Mod,
            ),
        }
    }
}

impl IRExpBuildable for UnaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            UnaryExp::PrimaryExp(exp) => exp.build(program, my_ir_generator_info),
            UnaryExp::PlusUnaryExp(exp) => exp.build(program, my_ir_generator_info),
            UnaryExp::MinusUnaryExp(exp) => build_binary_from_build_results(
                IRExpBuildResult::Const(0),
                exp.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(exp) => build_binary_from_build_results(
                IRExpBuildResult::Const(0),
                exp.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Eq,
            ),
        }
    }
}

impl IRExpBuildable for PrimaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            PrimaryExp::BracedExp(exp) => exp.build(program, my_ir_generator_info),
            PrimaryExp::LVal(lval) => {
                let result = lval.build(program, my_ir_generator_info)?;
                match result {
                    IRExpBuildResult::Const(_int) => Ok(result),
                    IRExpBuildResult::Value(ptr) => {
                        // If the PrimaryExp is a variable, then load it.
                        let load_inst = create_new_value(program, my_ir_generator_info).load(ptr);
                        insert_instructions(program, my_ir_generator_info, [load_inst]);
                        Ok(IRExpBuildResult::Value(load_inst))
                    }
                }
            }
            PrimaryExp::Number(number) => number.build(program, my_ir_generator_info),
        }
    }
}

impl IRExpBuildable for LVal {
    fn build(
        &self,
        _program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            LVal::IDENT(ident) => match my_ir_generator_info.symbol_tables.get(&ident.content) {
                Some(SymbolTableEntry::Variable(_lval_type, ptr)) => {
                    Ok(IRExpBuildResult::Value(*ptr))
                }
                Some(SymbolTableEntry::Constant(_lval_type, values)) => {
                    Ok(IRExpBuildResult::Const(values[0]))
                }
                None => Err(format!("Undeclared symbol: {}", ident.content)),
            },
        }
    }
}

impl IRExpBuildable for Number {
    fn build(
        &self,
        _program: &mut Program,
        _my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRExpBuildResult, String> {
        match self {
            Number::INTCONST(int) => Ok(IRExpBuildResult::Const(*int)),
        }
    }
}
