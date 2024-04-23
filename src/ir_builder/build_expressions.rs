//! Build a single component into Koopa IR.

use crate::ast_def::expressions::*;
use koopa::ir::{builder_traits::*, Program};

use super::{IRBuildResult, IRBuildable, MyIRGeneratorInfo, SymbolTableEntry, create_new_value, insert_instructions};

impl IRBuildable for Exp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            Exp::LOrExp(lor_exp) => lor_exp.build(program, my_ir_generator_info),
        }
    }
}

fn build_binary_from_build_results(
    result1: IRBuildResult,
    result2: IRBuildResult,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<IRBuildResult, String> {
    // If both expressions are constant expressions, then the result should be a constant expression.
    if let (IRBuildResult::Const(int1), IRBuildResult::Const(int2)) = (&result1, &result2) {
        Ok(IRBuildResult::Const(match binary_op {
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
            IRBuildResult::Const(int) => {
                create_new_value(program, my_ir_generator_info).integer(int)
            }
            IRBuildResult::Value(value) => value,
        };
        let value2 = match result2 {
            IRBuildResult::Const(int) => {
                create_new_value(program, my_ir_generator_info).integer(int)
            }
            IRBuildResult::Value(value) => value,
        };
        let new_value =
            create_new_value(program, my_ir_generator_info).binary(binary_op, value1, value2);
        insert_instructions(program, my_ir_generator_info, [new_value]);
        Ok(IRBuildResult::Value(new_value))
    }
}

impl IRBuildable for LOrExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            LOrExp::LAndExp(exp) => exp.build(program, my_ir_generator_info),
            LOrExp::BinaryLOrExp(exp1, exp2) => {
                let result1 = build_binary_from_build_results(
                    exp1.build(program, my_ir_generator_info)?,
                    IRBuildResult::Const(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let result2 = build_binary_from_build_results(
                    IRBuildResult::Const(0),
                    exp2.build(program, my_ir_generator_info)?,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                build_binary_from_build_results(
                    result1,
                    result2,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::Or,
                )
            }
        }
    }
}

impl IRBuildable for LAndExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            LAndExp::EqExp(exp) => exp.build(program, my_ir_generator_info),
            LAndExp::BinaryLAndExp(exp1, exp2) => {
                let result1 = build_binary_from_build_results(
                    exp1.build(program, my_ir_generator_info)?,
                    IRBuildResult::Const(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let result2 = build_binary_from_build_results(
                    IRBuildResult::Const(0),
                    exp2.build(program, my_ir_generator_info)?,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                build_binary_from_build_results(
                    result1,
                    result2,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::And,
                )
            }
        }
    }
}

impl IRBuildable for EqExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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

impl IRBuildable for RelExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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

impl IRBuildable for AddExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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

impl IRBuildable for MulExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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

impl IRBuildable for UnaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            UnaryExp::PrimaryExp(exp) => exp.build(program, my_ir_generator_info),
            UnaryExp::PlusUnaryExp(exp) => exp.build(program, my_ir_generator_info),
            UnaryExp::MinusUnaryExp(exp) => build_binary_from_build_results(
                IRBuildResult::Const(0),
                exp.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(exp) => build_binary_from_build_results(
                IRBuildResult::Const(0),
                exp.build(program, my_ir_generator_info)?,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Eq,
            ),
        }
    }
}

impl IRBuildable for PrimaryExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            PrimaryExp::BracedExp(exp) => exp.build(program, my_ir_generator_info),
            PrimaryExp::LVal(lval) => {
                let result = lval.build(program, my_ir_generator_info)?;
                match result {
                    IRBuildResult::Const(_int) => Ok(result),
                    IRBuildResult::Value(ptr) => {
                        // If the PrimaryExp is a variable, then load it. 
                        let load_inst = create_new_value(program, my_ir_generator_info).load(ptr);
                        insert_instructions(program, my_ir_generator_info, [load_inst]);
                        Ok(IRBuildResult::Value(load_inst))
                    }
                }
            }
            PrimaryExp::Number(number) => number.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for LVal {
    fn build(
        &self,
        _program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            LVal::IDENT(ident) => match my_ir_generator_info.symbol_table.get(&ident.content) {
                Some(SymbolTableEntry::Variable(_lval_type, ptr)) => Ok(IRBuildResult::Value(*ptr)),
                Some(SymbolTableEntry::Constant(_lval_type, values)) => {
                    Ok(IRBuildResult::Const(values[0]))
                }
                None => Err(format!("Undeclared symbol: {}", ident.content)),
            },
        }
    }
}

impl IRBuildable for Number {
    fn build(
        &self,
        _program: &mut Program,
        _my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            Number::INTCONST(int) => Ok(IRBuildResult::Const(*int)),
        }
    }
}
