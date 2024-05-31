//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    create_new_value, insert_instructions, IRBuildResult, IRBuildable, MyIRGeneratorInfo,
    SymbolTableEntry,
};

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let FuncDef::Default(return_type, func_id, params, block) = self;
        let return_type = Type::get(return_type.content.clone());
        let func = program.new_func(FunctionData::with_param_names(
            format!("@{}", func_id.content),
            vec![],
            return_type,
        ));
        // Insert the function name into symbol table.
        // TODO: Maybe we should use another symbol table for functions.
        let func_data = program.func_mut(func);
        let new_block = func_data
            .dfg_mut()
            .new_bb()
            .basic_block(None);
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
        let mut block_result = IRBuildResult::OK;
        my_ir_generator_info.symbol_tables.add_new_table();
        for stmt in stmts {
            let result = stmt.build(program, my_ir_generator_info)?;
            // Ignore everything after the return statement.
            if let IRBuildResult::EARLYSTOPPING = result {
                block_result = result;
                break;
            }
        }
        // for (_name, entry) in my_ir_generator_info
        //     .symbol_tables
        //     .symbol_tables
        //     .last()
        //     .unwrap()
        // {
        //     if let SymbolTableEntry::Variable(_tk, value) = entry {
        //         program
        //             .func_mut(my_ir_generator_info.curr_func.unwrap())
        //             .dfg_mut()
        //             .remove_value(*value);
        //     }
        // }
        my_ir_generator_info.symbol_tables.delete_new_table();
        Ok(block_result)
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
                IRExpBuildResult::Const(int) => {
                    my_ir_generator_info.symbol_tables.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Constant(const_type.clone(), vec![int]),
                    );
                }
                IRExpBuildResult::Value(_) => {
                    return Err(format!(
                        "Non-constant expression in constant declaration: {:?}",
                        const_def
                    ))
                }
            }
        }
        Ok(IRBuildResult::OK)
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
            let VarDef::Default(ident, possible_rhs) = var_def;
            // Allocate the new variable and get its Koopa IR Value.
            let var_ptr =
                create_new_value(program, my_ir_generator_info).alloc(Type::get(var_type.clone()));
            program
                .func_mut(my_ir_generator_info.curr_func.unwrap())
                .dfg_mut()
                .set_value_name(
                    var_ptr,
                    Some(format!(
                        "@{}_{}",
                        ident.content,
                        my_ir_generator_info.symbol_tables.curr_depth()
                    )),
                );
            // Insert the "alloc" instruction.
            insert_instructions(program, my_ir_generator_info, [var_ptr]);
            if let Some(rhs) = possible_rhs {
                // Build RHS value (if exists).
                let result = rhs.build(program, my_ir_generator_info)?;
                let rhs_value = match result {
                    IRExpBuildResult::Const(int) => {
                        create_new_value(program, my_ir_generator_info).integer(int)
                    }
                    IRExpBuildResult::Value(value) => value,
                };
                // Assign the RHS value into the new variable.
                let store_inst =
                    create_new_value(program, my_ir_generator_info).store(rhs_value, var_ptr);
                insert_instructions(program, my_ir_generator_info, [store_inst]);
            }
            // Add an entry in the symbol table.
            my_ir_generator_info.symbol_tables.insert(
                ident.content.clone(),
                SymbolTableEntry::Variable(var_type.clone(), var_ptr),
            );
        }
        Ok(IRBuildResult::OK)
    }
}
