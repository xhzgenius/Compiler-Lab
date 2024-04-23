//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::{IRBuildable, MyIRGeneratorInfo, SymbolTableEntry};

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let FuncDef::Default(return_type, func_id, block) = self;
        let return_type = Type::get(return_type.content.clone());
        // dbg!("Building function", &self);
        let func = program.new_func(FunctionData::with_param_names(
            "@".to_string() + func_id.content.as_str(),
            vec![],
            return_type,
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

impl IRBuildable for Block {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let Block::Default(stmts) = self;
        for stmt in stmts {
            stmt.build(program, my_ir_generator_info)?
        }
        Ok(())
    }
}

impl IRBuildable for BlockItem {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            BlockItem::Decl(decl) => decl.build(program, my_ir_generator_info)?,
            BlockItem::Stmt(stmt) => stmt.build(program, my_ir_generator_info)?,
        }
        my_ir_generator_info.curr_value = None;
        Ok(())
    }
}

impl IRBuildable for Decl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        match self {
            Decl::ConstDecl(const_decl) => const_decl.build(program, my_ir_generator_info),
            Decl::VarDecl(var_decl) => var_decl.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for ConstDecl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let ConstDecl::Default(btype, const_defs) = self;
        let const_type = &btype.content;
        for const_def in const_defs {
            let ConstDef::Default(ident, rhs) = const_def;
            // Start to calculate a constant.
            my_ir_generator_info.tmp_constants = Some((114514, 1919810));
            // Build RHS value.
            rhs.build(program, my_ir_generator_info)?;
            // Constant calculation finished.
            dbg!(my_ir_generator_info.tmp_constants);
            let (ans, _) = my_ir_generator_info.tmp_constants.unwrap();
            my_ir_generator_info.tmp_constants = None;
            // Add an entry in the symbol table.
            my_ir_generator_info.symbol_table.insert(
                ident.content.clone(),
                SymbolTableEntry::Constant(const_type.clone(), vec![ans]),
            );
        }
        Ok(())
    }
}

impl IRBuildable for ConstInitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let ConstInitVal::ConstExp(c) = self;
        c.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for ConstExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let ConstExp::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)?;
        Ok(())
    }
}

impl IRBuildable for VarDecl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let VarDecl::Default(btype, var_defs) = self;
        let var_type = &btype.content;
        for var_def in var_defs {
            match var_def {
                VarDef::WithInitVal(ident, rhs) => {
                    // Build RHS value.
                    rhs.build(program, my_ir_generator_info)?;
                    let rhs_value = my_ir_generator_info.curr_value.unwrap();
                    // Allocate the new variable.
                    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                    let dfg = curr_func_data.dfg_mut();
                    let var_ptr = dfg.new_value().alloc(Type::get(var_type.clone()));
                    // Assign the RHS value into the new variable.
                    let store_inst = dfg.new_value().store(rhs_value, var_ptr);
                    dfg.set_value_name(var_ptr, Some(format!("@{}", ident.content)));
                    // Add an entry in the symbol table.
                    my_ir_generator_info.symbol_table.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Variable(var_type.clone(), var_ptr),
                    );
                    curr_func_data
                        .layout_mut()
                        .bb_mut(my_ir_generator_info.curr_block.unwrap())
                        .insts_mut()
                        .extend([var_ptr, store_inst])
                }
                VarDef::WithoutInitVal(ident) => {
                    // Allocate the new variable.
                    let curr_func_data = program.func_mut(my_ir_generator_info.curr_func.unwrap());
                    let dfg = curr_func_data.dfg_mut();
                    let var_ptr = dfg.new_value().alloc(Type::get(var_type.clone()));
                    // Add an entry in the symbol table.
                    my_ir_generator_info.symbol_table.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Variable(var_type.clone(), var_ptr),
                    );
                    curr_func_data
                        .layout_mut()
                        .bb_mut(my_ir_generator_info.curr_block.unwrap())
                        .insts_mut()
                        .extend([var_ptr])
                }
            }
        }
        Ok(())
    }
}

impl IRBuildable for InitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<(), String> {
        let InitVal::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)
    }
}
