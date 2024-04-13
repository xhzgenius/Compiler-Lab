//! Build a single component into Koopa IR. 

use koopa::ir::{Program, FunctionData, Type, builder_traits::*};
use crate::ast_def::*;

use super::MyIRGeneratorInfo;

pub trait Buildable {
    fn build(&self, program: &mut Program, my_ir_generator_info: &mut MyIRGeneratorInfo) -> Result<(), &str>;
}

impl Buildable for CompUnit {
    fn build(&self, program: &mut Program, my_ir_generator_info: &mut MyIRGeneratorInfo) -> Result<(), &str> {
        self.func_def.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl Buildable for FuncDef {
    fn build(&self, program: &mut Program, my_ir_generator_info: &mut MyIRGeneratorInfo) -> Result<(), &str> {
        let return_type = match self.return_type.type_name.as_str(){
            "int" => Ok(Type::get_i32()),
            _ => Err("Wrong return type")
        };
        dbg!("Building function", &self);
        let func = program.new_func(
            FunctionData::with_param_names(
                "@".to_string()+self.func_id.as_str(), 
                vec![], 
                return_type.expect("Error creating new function")
            )
        );
        let func_data = program.func_mut(func);
        let new_block = func_data.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));
        func_data.layout_mut().bbs_mut().extend([new_block]);
        my_ir_generator_info.curr_block = Some(new_block);
        my_ir_generator_info.curr_func = Some(func);
        self.block.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl Buildable for Block {
    fn build(&self, program: &mut Program, my_ir_generator_info: &mut MyIRGeneratorInfo) -> Result<(), &str> {
        self.stmt.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl Buildable for Stmt {
    fn build(&self, program: &mut Program, my_ir_generator_info: &mut MyIRGeneratorInfo) -> Result<(), &str> {
        match &self.stmt {
            StmtEnum::ReturnStmt(number) => {
                let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                let return_value = match number.number {
                    NumberEnum::IntConst(int_const) => curr_func_data.dfg_mut().new_value().integer(int_const),
                };
                let return_stmt = curr_func_data.dfg_mut().new_value().ret(Some(return_value));
                curr_func_data.layout_mut().bb_mut(my_ir_generator_info.curr_block.unwrap()).insts_mut().extend([return_stmt]);
            }
        }
        Ok(())
    }

}