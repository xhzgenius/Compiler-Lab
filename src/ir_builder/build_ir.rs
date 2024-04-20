//! Build a single component into Koopa IR.

use crate::ast_def::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::MyIRGeneratorInfo;

pub trait IRBuildable {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String>;
}

impl IRBuildable for CompUnit {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        self.func_def.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let return_type = match self.return_type.type_name.as_str() {
            "int" => Ok(Type::get_i32()),
            _ => Err("Wrong return type"),
        };
        // dbg!("Building function", &self);
        let func = program.new_func(FunctionData::with_param_names(
            "@".to_string() + self.func_id.as_str(),
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
        self.block.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl IRBuildable for Block {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Block::Stmt(stmt) => stmt.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for Stmt {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match &self {
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

fn build_binary_from_values(
    first_value: koopa::ir::Value,
    second_value: koopa::ir::Value,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
    binary_op: koopa::ir::BinaryOp,
) -> Result<(), String> {
    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
    let new_value =
        curr_func_data
            .dfg_mut()
            .new_value()
            .binary(binary_op, first_value, second_value);
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
    first_exp.build(program, my_ir_generator_info)?;
    let first_value = my_ir_generator_info
        .curr_value
        .expect("No curr_value. Should not happen. ");
    second_exp.build(program, my_ir_generator_info)?;
    let second_value = my_ir_generator_info
        .curr_value
        .expect("No curr_value. Should not happen. ");
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
                    &Number::IntConst(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let bool1 = my_ir_generator_info
                    .curr_value
                    .expect("No curr_value. Should not happen. ");
                build_binary_from_buildables(
                    &Number::IntConst(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let bool2 = my_ir_generator_info
                    .curr_value
                    .expect("No curr_value. Should not happen. ");
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
                    &Number::IntConst(0),
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let bool1 = my_ir_generator_info
                    .curr_value
                    .expect("No curr_value. Should not happen. ");
                build_binary_from_buildables(
                    &Number::IntConst(0),
                    second_exp,
                    program,
                    my_ir_generator_info,
                    koopa::ir::BinaryOp::NotEq,
                )?;
                let bool2 = my_ir_generator_info
                    .curr_value
                    .expect("No curr_value. Should not happen. ");
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
                &Number::IntConst(0),
                &**unary_exp,
                program,
                my_ir_generator_info,
                koopa::ir::BinaryOp::Sub,
            ),
            UnaryExp::NotUnaryExp(unary_exp) => build_binary_from_buildables(
                &Number::IntConst(0),
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
            PrimaryExp::Number(number) => number.build(program, my_ir_generator_info),
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
            Number::IntConst(int) => {
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                my_ir_generator_info.curr_value =
                    Some(curr_func_data.dfg_mut().new_value().integer(*int));
                Ok(())
            }
        }
    }
}
