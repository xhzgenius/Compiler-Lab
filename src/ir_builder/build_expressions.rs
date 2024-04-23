//! Build a single component into Koopa IR.

use crate::ast_def::expressions::*;
use koopa::ir::{builder_traits::*, Program};

use super::{IRBuildable, MyIRGeneratorInfo, SymbolTableEntry};

impl IRBuildable for Exp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Exp::LOrExp(lor_exp) => lor_exp.build(program, my_ir_generator_info),
        }
    }
}

/// The two values may be None if calculating a constant expression.
fn build_binary_from_values(
    first_value: Option<koopa::ir::Value>,
    second_value: Option<koopa::ir::Value>,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    if let Some((tmp1, tmp2)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        match binary_op {
            koopa::ir::BinaryOp::NotEq => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 != tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Eq => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 == tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Gt => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 > tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Lt => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 < tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Ge => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 >= tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Le => {
                my_ir_generator_info.tmp_constants = Some(((tmp1 <= tmp2) as i32, 114514))
            }
            koopa::ir::BinaryOp::Add => {
                my_ir_generator_info.tmp_constants = Some((tmp1 + tmp2, 114514))
            }
            koopa::ir::BinaryOp::Sub => {
                my_ir_generator_info.tmp_constants = Some((tmp1 - tmp2, 114514))
            }
            koopa::ir::BinaryOp::Mul => {
                my_ir_generator_info.tmp_constants = Some((tmp1 * tmp2, 114514))
            }
            koopa::ir::BinaryOp::Div => {
                my_ir_generator_info.tmp_constants = Some((tmp1 / tmp2, 114514))
            }
            koopa::ir::BinaryOp::Mod => {
                my_ir_generator_info.tmp_constants = Some((tmp1 % tmp2, 114514))
            }
            koopa::ir::BinaryOp::And => {
                my_ir_generator_info.tmp_constants = Some((tmp1 & tmp2, 114514))
            }
            koopa::ir::BinaryOp::Or => {
                my_ir_generator_info.tmp_constants = Some((tmp1 | tmp2, 114514))
            }
            koopa::ir::BinaryOp::Xor => {
                my_ir_generator_info.tmp_constants = Some((tmp1 ^ tmp2, 114514))
            }
            koopa::ir::BinaryOp::Shl => {
                my_ir_generator_info.tmp_constants = Some((tmp1 << tmp2, 114514))
            }
            koopa::ir::BinaryOp::Shr => todo!(), // SysY has no left shift or right shift.
            koopa::ir::BinaryOp::Sar => {
                my_ir_generator_info.tmp_constants = Some((tmp1 >> tmp2, 114514))
            }
        }
        return Ok(());
    }
    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
    let new_value = curr_func_data.dfg_mut().new_value().binary(
        binary_op,
        first_value.unwrap(),
        second_value.unwrap(),
    );
    curr_func_data
        .layout_mut()
        .bb_mut(my_ir_generator_info.curr_block.unwrap())
        .insts_mut()
        .extend([new_value]);
    my_ir_generator_info.curr_value = Some(new_value);
    Ok(())
}

fn build_binary_from_buildables(
    first_exp: &dyn IRBuildable,
    second_exp: &dyn IRBuildable,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    let mut tmp1 = 0;
    first_exp.build(program, my_ir_generator_info)?;
    let first_value = my_ir_generator_info.curr_value;
    if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        tmp1 = tmp;
    }
    second_exp.build(program, my_ir_generator_info)?;
    let second_value = my_ir_generator_info.curr_value;
    if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
        // Calculating constant expression
        my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
    }
    build_binary_from_values(
        first_value,
        second_value,
        program,
        my_ir_generator_info,
        binary_op,
    )
}

impl IRBuildable for LOrExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            LOrExp::LAndExp(land_exp) => land_exp.build(program, my_ir_generator_info),
            LOrExp::BinaryLOrExp(first_exp, second_exp) => {
                build_binary_from_buildables(
                    &**first_exp,
                    &Number::INTCONST(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let mut tmp1 = 0;
                if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    tmp1 = tmp;
                }
                let bool1 = my_ir_generator_info.curr_value;
                build_binary_from_buildables(
                    &Number::INTCONST(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
                }
                let bool2 = my_ir_generator_info.curr_value;
                build_binary_from_values(
                    bool1,
                    bool2,
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
    ) -> Result<(), String> {
        match self {
            LAndExp::EqExp(second_exp) => second_exp.build(program, my_ir_generator_info),
            LAndExp::BinaryLAndExp(first_exp, second_exp) => {
                build_binary_from_buildables(
                    &**first_exp,
                    &Number::INTCONST(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let mut tmp1 = 0;
                if let Some((tmp, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    tmp1 = tmp;
                }
                let bool1 = my_ir_generator_info.curr_value;
                build_binary_from_buildables(
                    &Number::INTCONST(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                if let Some((tmp2, _)) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((tmp1, tmp2));
                }
                let bool2 = my_ir_generator_info.curr_value;
                build_binary_from_values(
                    bool1,
                    bool2,
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
    ) -> Result<(), String> {
        match self {
            EqExp::RelExp(second_exp) => second_exp.build(program, my_ir_generator_info),
            EqExp::BinaryEqExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Eq,
            ),
            EqExp::BinaryUneqExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
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
    ) -> Result<(), String> {
        match self {
            RelExp::AddExp(second_exp) => second_exp.build(program, my_ir_generator_info),
            RelExp::BinaryLtExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Lt,
            ),
            RelExp::BinaryGtExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Gt,
            ),
            RelExp::BinaryLeExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Le,
            ),
            RelExp::BinaryGeExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
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
    ) -> Result<(), String> {
        match self {
            AddExp::MulExp(second_exp) => second_exp.build(program, my_ir_generator_info),
            AddExp::BinaryAddExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Add,
            ),
            AddExp::BinarySubExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
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
    ) -> Result<(), String> {
        match self {
            MulExp::UnaryExp(second_exp) => second_exp.build(program, my_ir_generator_info),
            MulExp::BinaryMulExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Mul,
            ),
            MulExp::BinaryDivExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Div,
            ),
            MulExp::BinaryModExp(first_exp, second_exp) => build_binary_from_buildables(
                &**first_exp,
                second_exp,
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
    ) -> Result<(), String> {
        match self {
            UnaryExp::PrimaryExp(primary_exp) => primary_exp.build(program, my_ir_generator_info),
            UnaryExp::PlusUnaryExp(unary_exp) => unary_exp.build(program, my_ir_generator_info),
            UnaryExp::MinusUnaryExp(unary_exp) => build_binary_from_buildables(
                &Number::INTCONST(0),
                &**unary_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(unary_exp) => build_binary_from_buildables(
                &Number::INTCONST(0),
                &**unary_exp,
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
    ) -> Result<(), String> {
        match self {
            PrimaryExp::BracedExp(exp) => exp.build(program, my_ir_generator_info),
            PrimaryExp::LVal(lval) => {
                let LVal::IDENT(ident) = lval;
                // IDENT be variable or constant.
                match my_ir_generator_info.symbol_table.get(&ident.content) {
                    Some(SymbolTableEntry::Variable(..)) => {
                        lval.build(program, my_ir_generator_info)?;
                        // Load the variable from the pointer.
                        let ptr = my_ir_generator_info.curr_value.unwrap();
                        let curr_func_data =
                            program.func_mut(my_ir_generator_info.curr_func.unwrap());
                        let dfg = curr_func_data.dfg_mut();
                        let load_inst = dfg.new_value().load(ptr);
                        my_ir_generator_info.curr_value = Some(load_inst);
                        curr_func_data
                            .layout_mut()
                            .bb_mut(my_ir_generator_info.curr_block.unwrap())
                            .insts_mut()
                            .extend([load_inst]);
                        Ok(())
                    }
                    Some(SymbolTableEntry::Constant(..)) => {
                        lval.build(program, my_ir_generator_info)
                    }
                    None => Err(format!("Undeclared symbol: {}", ident.content)),
                }
            }
            PrimaryExp::Number(number) => number.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for LVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            LVal::IDENT(ident) => match my_ir_generator_info.symbol_table.get(&ident.content) {
                Some(SymbolTableEntry::Variable(_lval_type, ptr)) => {
                    if let Some(_) = my_ir_generator_info.tmp_constants {
                        // Calculating constant expression
                        return Err(format!("Left Value should not exist in const expression! "));
                    }
                    // Don't load it right now, because it may be used as a pointer.
                    my_ir_generator_info.curr_value = Some(*ptr);
                    Ok(())
                }
                Some(SymbolTableEntry::Constant(_lval_type, values)) => {
                    if let Some(_) = my_ir_generator_info.tmp_constants {
                        // Calculating constant expression
                        my_ir_generator_info.tmp_constants = Some((values[0], 114514));
                        return Ok(());
                    }
                    my_ir_generator_info.curr_value = Some(
                        program
                            .func_mut(my_ir_generator_info.curr_func.unwrap())
                            .dfg_mut()
                            .new_value()
                            .integer(values[0]),
                    );
                    Ok(())
                }
                None => Err(format!("Undeclared symbol: {}", ident.content)),
            },
        }
    }
}

impl IRBuildable for Number {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Number::INTCONST(int) => {
                if let Some(_) = my_ir_generator_info.tmp_constants {
                    // Calculating constant expression
                    my_ir_generator_info.tmp_constants = Some((*int, 114514));
                    return Ok(());
                }
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                my_ir_generator_info.curr_value =
                    Some(curr_func_data.dfg_mut().new_value().integer(*int));
                Ok(())
            }
        }
    }
}
