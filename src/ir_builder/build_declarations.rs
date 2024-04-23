//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::{
    create_new_value, insert_instructions, IRBuildResult, IRBuildable, MyIRGeneratorInfo,
    SymbolTableEntry,
};

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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
        block.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for Block {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let Block::Default(stmts) = self;
        for stmt in stmts {
            stmt.build(program, my_ir_generator_info)?;
        }
        Ok(IRBuildResult::Const(114514))
    }
}

impl IRBuildable for BlockItem {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        match self {
            BlockItem::Decl(decl) => decl.build(program, my_ir_generator_info),
            BlockItem::Stmt(stmt) => stmt.build(program, my_ir_generator_info),
        }
    }
}

impl IRBuildable for Decl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
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
    ) -> Result<IRBuildResult, String> {
        let ConstDecl::Default(btype, const_defs) = self;
        let const_type = &btype.content;
        for const_def in const_defs {
            let ConstDef::Default(ident, rhs) = const_def;
            let result = rhs.build(program, my_ir_generator_info)?;
            // Add an entry in the symbol table.
            match result {
                IRBuildResult::Const(int) => {
                    my_ir_generator_info.symbol_table.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Constant(const_type.clone(), vec![int]),
                    );
                }
                IRBuildResult::Value(_) => {
                    return Err(format!(
                        "Non-constant expression in constant declaration: {:?}",
                        const_def
                    ))
                }
            }
        }
        Ok(IRBuildResult::Const(114514))
    }
}

impl IRBuildable for ConstInitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let ConstInitVal::ConstExp(c) = self;
        c.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for ConstExp {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let ConstExp::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)
    }
}

impl IRBuildable for VarDecl {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let VarDecl::Default(btype, var_defs) = self;
        let var_type = &btype.content;
        for var_def in var_defs {
            match var_def {
                VarDef::WithInitVal(ident, rhs) => {
                    // Build RHS value.
                    let result = rhs.build(program, my_ir_generator_info)?;
                    let rhs_value = match result {
                        IRBuildResult::Const(int) => {
                            create_new_value(program, my_ir_generator_info).integer(int)
                        }
                        IRBuildResult::Value(value) => value,
                    };
                    // Allocate the new variable.
                    let var_ptr = create_new_value(program, my_ir_generator_info)
                        .alloc(Type::get(var_type.clone()));
                    // Assign the RHS value into the new variable.
                    let store_inst =
                        create_new_value(program, my_ir_generator_info).store(rhs_value, var_ptr);
                    program
                        .func_mut(my_ir_generator_info.curr_func.unwrap())
                        .dfg_mut()
                        .set_value_name(var_ptr, Some(format!("@{}", ident.content)));
                    insert_instructions(program, my_ir_generator_info, [var_ptr, store_inst]);
                    // Add an entry in the symbol table.
                    my_ir_generator_info.symbol_table.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Variable(var_type.clone(), var_ptr),
                    );
                }
                VarDef::WithoutInitVal(ident) => {
                    // Allocate the new variable.
                    let var_ptr = create_new_value(program, my_ir_generator_info)
                        .alloc(Type::get(var_type.clone()));
                    program
                        .func_mut(my_ir_generator_info.curr_func.unwrap())
                        .dfg_mut()
                        .set_value_name(var_ptr, Some(format!("@{}", ident.content)));
                    insert_instructions(program, my_ir_generator_info, [var_ptr]);
                    // Add an entry in the symbol table.
                    my_ir_generator_info.symbol_table.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Variable(var_type.clone(), var_ptr),
                    );
                }
            }
        }
        Ok(IRBuildResult::Const(114514))
    }
}

impl IRBuildable for InitVal {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let InitVal::Exp(exp) = self;
        exp.build(program, my_ir_generator_info)
    }
}
