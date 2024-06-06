//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    create_new_local_value, insert_local_instructions, IRBuildResult, IRBuildable,
    MyIRGeneratorInfo, SymbolTableEntry,
};

impl IRBuildable for FuncDef {
    fn build(
        &self,
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRBuildResult, String> {
        let FuncDef::Default(return_type, func_id, params, block) = self;
        // Tell Koopa IR its return type and params.
        let return_type = Type::get(return_type.content.clone());
        let mut koopa_ir_params = Vec::<(Option<String>, Type)>::new();
        for FuncFParam::Default(btype, ident) in params {
            koopa_ir_params.push((
                Some(format!("%{}_param", &ident.content)),
                Type::get(btype.content.clone()),
            ));
        }
        let func = program.new_func(FunctionData::with_param_names(
            format!("@{}", func_id.content),
            koopa_ir_params,
            return_type,
        ));

        // Insert the function name into symbol table.
        // Check whether there are two global symbols with the same name.
        if my_ir_generator_info.check_duplicate_global_symbol(&func_id.content) {
            return Err(format!(
                "Invalid function name: {}. There was another global symbol with the same name! ",
                func_id.content
            ));
        }
        my_ir_generator_info
            .function_table
            .insert(func_id.content.clone(), func);

        /* Create a new BasicBlock and:
           - Allocate form params;
           - Insert form params into symbol table;
           - Assign real params to form params.
        */
        let func_data = program.func_mut(func);
        let new_block = func_data.dfg_mut().new_bb().basic_block(None);
        func_data.layout_mut().bbs_mut().extend([new_block]);
        my_ir_generator_info.curr_block = Some(new_block);
        my_ir_generator_info.curr_func = Some(func);

        my_ir_generator_info.symbol_tables.add_new_table();
        for idx in 0..program.func(func).params().len() {
            let FuncFParam::Default(btype, ident) = &params[idx];
            let real_param = program.func(func).params()[idx];
            // Allocate form params.
            let form_param = create_new_local_value(program, my_ir_generator_info)
                .alloc(Type::get(btype.content.clone()));
            program
                .func_mut(func)
                .dfg_mut()
                .set_value_name(form_param, Some(format!("@{}", ident.content,)));
            // Insert form params into symbol table.
            my_ir_generator_info.symbol_tables.insert(
                ident.content.clone(),
                SymbolTableEntry::Variable(btype.content.clone(), form_param),
            );
            // Assign real params to form params.
            let assign_inst =
                create_new_local_value(program, my_ir_generator_info).store(real_param, form_param);
            insert_local_instructions(program, my_ir_generator_info, [form_param, assign_inst]);
        }

        // Build the function body.
        match block.build(program, my_ir_generator_info)? {
            IRBuildResult::OK => {
                // No return instruction. Add a return instruction.
                let return_inst = create_new_local_value(program, my_ir_generator_info).ret(None);
                insert_local_instructions(program, my_ir_generator_info, [return_inst]);
            }
            IRBuildResult::EARLYSTOPPING => {}
        }

        my_ir_generator_info.curr_func = None;
        my_ir_generator_info.curr_block = None;
        Ok(IRBuildResult::OK)
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
        for stmt in stmts {
            let result = stmt.build(program, my_ir_generator_info)?;
            // Ignore everything after the return statement.
            if let IRBuildResult::EARLYSTOPPING = result {
                block_result = result;
                break;
            }
        }
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
            let final_var_ptr = match my_ir_generator_info.curr_func {
                // If it's local:
                Some(func) => {
                    let var_ptr = create_new_local_value(program, my_ir_generator_info)
                        .alloc(Type::get(var_type.clone()));
                    program
                        .func_mut(func)
                        .dfg_mut()
                        .set_value_name(var_ptr, Some(format!("@{}", ident.content,)));
                    // Insert the "alloc" instruction.
                    insert_local_instructions(program, my_ir_generator_info, [var_ptr]);
                    if let Some(rhs) = possible_rhs {
                        // Build RHS value (if exists).
                        let result = rhs.build(program, my_ir_generator_info)?;
                        let rhs_value = match result {
                            IRExpBuildResult::Const(int) => {
                                create_new_local_value(program, my_ir_generator_info).integer(int)
                            }
                            IRExpBuildResult::Value(value) => value,
                        };
                        // Assign the RHS value into the new variable.
                        let store_inst = create_new_local_value(program, my_ir_generator_info)
                            .store(rhs_value, var_ptr);
                        insert_local_instructions(program, my_ir_generator_info, [store_inst]);
                    }
                    var_ptr
                }
                // Or if it's global:
                None => {
                    // Check whether there are two global symbols with the same name.
                    if my_ir_generator_info.check_duplicate_global_symbol(&ident.content) {
                        return Err(format!(
                            "Invalid global variable name: {}. There was another global symbol with the same name! ",
                            ident.content
                        ));
                    }
                    let var_ptr = match possible_rhs {
                        Some(rhs) => match rhs.build(program, my_ir_generator_info)? {
                            IRExpBuildResult::Const(int) => {
                                let int_init = program.new_value().integer(int);
                                program.new_value().global_alloc(int_init)
                            }
                            IRExpBuildResult::Value(val) => program.new_value().global_alloc(val),
                        },
                        None => {
                            let zero_init =
                                program.new_value().zero_init(Type::get(var_type.clone()));
                            program.new_value().global_alloc(zero_init)
                        }
                    };
                    program.set_value_name(var_ptr, Some(format!("@{}", ident.content,)));
                    var_ptr
                }
            };

            // Add an entry in the symbol table.
            my_ir_generator_info.symbol_tables.insert(
                ident.content.clone(),
                SymbolTableEntry::Variable(var_type.clone(), final_var_ptr),
            );
        }
        Ok(IRBuildResult::OK)
    }
}
