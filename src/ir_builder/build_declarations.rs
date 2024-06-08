//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type, Value};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    build_shape, create_new_local_value, get_array_type, insert_local_instructions, IRBuildResult,
    IRBuildable, MyIRGeneratorInfo, SymbolTableEntry,
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
        for FuncFParam::Default(btype, ident, possible_shape_exps) in params {
            let param_type = match possible_shape_exps {
                Some(shape_exps) => {
                    let shape = build_shape(shape_exps, program, my_ir_generator_info)?;
                    Type::get_pointer(Type::get(get_array_type(btype, &shape)))
                }
                None => Type::get(btype.content.clone()),
            };
            koopa_ir_params.push((Some(format!("%{}_param", &ident.content)), param_type));
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
            let FuncFParam::Default(btype, ident, possible_shape_exps) = &params[idx];
            let real_param = program.func(func).params()[idx];
            // Allocate form params.
            let form_param_type = match possible_shape_exps {
                Some(shape_exps) => {
                    let shape = build_shape(shape_exps, program, my_ir_generator_info)?;
                    Type::get_pointer(Type::get(get_array_type(btype, &shape)))
                }
                None => Type::get(btype.content.clone()),
            };
            let form_param =
                create_new_local_value(program, my_ir_generator_info).alloc(form_param_type);
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

pub enum IRInitValBuildResult {
    Const(i32),
    Var(Value),
    Aggregate(Value),
}

impl InitVal {
    fn build(
        &self,
        shape: &[usize],
        program: &mut Program,
        my_ir_generator_info: &mut MyIRGeneratorInfo,
    ) -> Result<IRInitValBuildResult, String> {
        let is_global = my_ir_generator_info.curr_func.is_none();
        match self {
            InitVal::Exp(exp) => match exp.build(program, my_ir_generator_info)? {
                IRExpBuildResult::Const(int) => Ok(IRInitValBuildResult::Const(int)),
                IRExpBuildResult::Value(value) => {
                    if is_global {
                        Err(format!(
                            "Non-constant expression in global variable initval: {:?}",
                            self
                        ))
                    } else {
                        Ok(IRInitValBuildResult::Var(value))
                    }
                }
            },
            InitVal::Aggregate(aggr) => {
                let mut elems = vec![];
                // let mut shape_cnt = Vec::<usize>::new();
                // for _ in 0..shape.len() {
                //     shape_cnt.push(0);
                // }
                for initval in aggr {
                    // Decide which dim to aggregate.
                    // let mut curr_possible_dim = shape.len();
                    // for dim in (0..shape.len()).rev() {
                    //     if shape_cnt[dim] == 0 {
                    //         curr_possible_dim -= 1;
                    //     } else {
                    //         break;
                    //     }
                    // }
                    let curr_element_shape = &shape[..shape.len() - 1];
                    // let curr_element_shape = &shape[curr_possible_dim..shape.len()];
                    let elem =
                        match initval.build(curr_element_shape, program, my_ir_generator_info)? {
                            IRInitValBuildResult::Const(int) => {
                                if is_global {
                                    program.new_value().integer(int)
                                } else {
                                    create_new_local_value(program, my_ir_generator_info)
                                        .integer(int)
                                }
                            }
                            IRInitValBuildResult::Var(_) => {
                                return Err(format!("Aggregate cannot have a variable in it!"))
                            }
                            IRInitValBuildResult::Aggregate(value) => value,
                        };
                    elems.push(elem);
                    // shape_cnt[curr_possible_dim] += 1;
                }
                if is_global {
                    Ok(IRInitValBuildResult::Aggregate(
                        program.new_value().aggregate(elems),
                    ))
                } else {
                    Ok(IRInitValBuildResult::Aggregate(
                        create_new_local_value(program, my_ir_generator_info).aggregate(elems),
                    ))
                }
            }
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
            let ConstDef::Default(ident, shape_exps, rhs) = const_def;
            let shape = build_shape(shape_exps, program, my_ir_generator_info)?.clone();
            let result = rhs.build(&shape, program, my_ir_generator_info)?;
            // Add an entry in the symbol table.
            match result {
                IRInitValBuildResult::Const(int) => {
                    my_ir_generator_info.symbol_tables.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Constant(const_type.clone(), int),
                    );
                }
                IRInitValBuildResult::Var(_) => {
                    return Err(format!(
                        "Non-constant expression in constant declaration: {:?}",
                        const_def
                    ))
                }
                IRInitValBuildResult::Aggregate(aggr) => {
                    let array_ptr = program.new_value().global_alloc(aggr);
                    program.set_value_name(array_ptr, Some(format!("@{}", ident.content,)));
                    my_ir_generator_info.symbol_tables.insert(
                        ident.content.clone(),
                        SymbolTableEntry::Variable(const_type.clone(), array_ptr),
                    );
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

        for var_def in var_defs {
            let VarDef::Default(ident, shape_exps, possible_rhs) = var_def;
            let shape = build_shape(shape_exps, program, my_ir_generator_info)?;
            let var_type = match shape.is_empty() {
                true => btype.content.clone(),
                false => get_array_type(btype, &shape),
            };

            // Allocate the new variable and get its Koopa IR Value.
            let final_var_addr = match my_ir_generator_info.curr_func {
                // If it's local:
                Some(func) => {
                    let var_addr = create_new_local_value(program, my_ir_generator_info)
                        .alloc(Type::get(var_type.clone()));
                    program
                        .func_mut(func)
                        .dfg_mut()
                        .set_value_name(var_addr, Some(format!("@{}", ident.content,)));
                    // Insert the "alloc" instruction.
                    insert_local_instructions(program, my_ir_generator_info, [var_addr]);
                    if let Some(rhs) = possible_rhs {
                        // Build RHS value (if exists).
                        let result = rhs.build(&shape, program, my_ir_generator_info)?;
                        let rhs_value = match result {
                            IRInitValBuildResult::Const(int) => {
                                create_new_local_value(program, my_ir_generator_info).integer(int)
                            }
                            IRInitValBuildResult::Var(value) => value,
                            IRInitValBuildResult::Aggregate(value) => value,
                        };
                        // Assign the RHS value into the new variable.
                        // dbg!(
                        //     get_valuedata(rhs_value, program, my_ir_generator_info),
                        //     get_valuedata(var_addr, program, my_ir_generator_info)
                        // );
                        let store_inst = create_new_local_value(program, my_ir_generator_info)
                            .store(rhs_value, var_addr);
                        insert_local_instructions(program, my_ir_generator_info, [store_inst]);
                    }
                    var_addr
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
                    let var_addr = match possible_rhs {
                        Some(rhs) => match rhs.build(&shape, program, my_ir_generator_info)? {
                            IRInitValBuildResult::Const(int) => {
                                let int_init = program.new_value().integer(int);
                                program.new_value().global_alloc(int_init)
                            }
                            IRInitValBuildResult::Var(val) => program.new_value().global_alloc(val),
                            IRInitValBuildResult::Aggregate(val) => {
                                program.new_value().global_alloc(val)
                            }
                        },
                        None => {
                            let zero_init =
                                program.new_value().zero_init(Type::get(var_type.clone()));
                            program.new_value().global_alloc(zero_init)
                        }
                    };
                    program.set_value_name(var_addr, Some(format!("@{}", ident.content,)));
                    var_addr
                }
            };

            // Add an entry in the symbol table.
            my_ir_generator_info.symbol_tables.insert(
                ident.content.clone(),
                SymbolTableEntry::Variable(var_type.clone(), final_var_addr),
            );
        }
        Ok(IRBuildResult::OK)
    }
}
