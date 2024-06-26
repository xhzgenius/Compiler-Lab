//! Build a single component into Koopa IR.

use crate::ast_def::declarations::*;
use koopa::ir::{builder_traits::*, FunctionData, Program, Type, Value};

use super::{
    build_expressions::{IRExpBuildResult, IRExpBuildable},
    build_shape, create_new_local_value, get_array_type, get_valuedata, insert_local_instructions,
    IRBuildResult, IRBuildable, MyIRGeneratorInfo, SymbolTableEntry,
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

fn build_new_aggregate(
    shape: &[usize],
    childs: &[Box<InitVal>],
    is_global: bool,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
) -> Result<(Value, usize), String> {
    let mut elems = vec![];
    let mut curr_child_idx = 0;
    if shape.len() == 1 {
        for _ in 0..shape[0] {
            // 1D array. Only contains Exps.
            let next_child = if curr_child_idx < childs.len() {
                Some(&*childs[curr_child_idx])
            } else {
                None
            };
            let result = match next_child {
                Some(InitVal::Exp(exp)) => exp.build(program, my_ir_generator_info)?,
                Some(InitVal::Aggregate(_)) => return Err(format!("Wrong aggregate")),
                None => IRExpBuildResult::Const(0),
            };
            curr_child_idx += 1;
            let value = match result {
                IRExpBuildResult::Const(int) => {
                    if is_global {
                        program.new_value().integer(int)
                    } else {
                        create_new_local_value(program, my_ir_generator_info).integer(int)
                    }
                }
                IRExpBuildResult::Value(_) => {
                    return Err(format!(
                        "Non-constant expression in aggregate initval: {:?}",
                        next_child
                    ))
                }
            };
            elems.push(value);
        }
    } else {
        for _ in 0..shape[0] {
            let next_child = if curr_child_idx < childs.len() {
                Some(&*childs[curr_child_idx])
            } else {
                None
            };
            match next_child {
                Some(InitVal::Exp(_)) => {}
                Some(InitVal::Aggregate(_)) => {
                    let result =
                        childs[curr_child_idx].build(&shape[1..], program, my_ir_generator_info)?;
                    match result {
                        IRInitValBuildResult::Const(_) | IRInitValBuildResult::Var(_) => {
                            panic!("Aggregate can only build aggregate!")
                        }
                        IRInitValBuildResult::Aggregate(aggr) => elems.push(aggr),
                    }
                    curr_child_idx += 1;
                    continue;
                }
                None => {}
            };
            let (result, used_child_cnt) = build_new_aggregate(
                &shape[1..],
                if curr_child_idx < childs.len() {
                    &childs[curr_child_idx..]
                } else {
                    &[]
                },
                is_global,
                program,
                my_ir_generator_info,
            )?;
            curr_child_idx += used_child_cnt;
            elems.push(result);
        }
    }
    if is_global {
        Ok((program.new_value().aggregate(elems), curr_child_idx))
    } else {
        Ok((
            create_new_local_value(program, my_ir_generator_info).aggregate(elems),
            curr_child_idx,
        ))
    }
}

fn aggregate_to_store_insts(
    aggr: Value,
    aggr_ptr: Value,
    program: &mut Program,
    my_ir_generator_info: &mut MyIRGeneratorInfo,
) -> Result<(), String> {
    assert!(
        !aggr_ptr.is_global(),
        "Global aggregate initialization cannot be converted into store instructions! "
    );
    let valuedata = get_valuedata(aggr, program, my_ir_generator_info);
    match valuedata.kind() {
        koopa::ir::ValueKind::Aggregate(aggr) => {
            for i in 0..aggr.elems().len() {
                let index = create_new_local_value(program, my_ir_generator_info).integer(i as i32);
                let child = aggr.elems()[i];
                let child_valuedata = get_valuedata(child, program, my_ir_generator_info);
                let child_ptr = create_new_local_value(program, my_ir_generator_info)
                    .get_elem_ptr(aggr_ptr, index);
                insert_local_instructions(program, my_ir_generator_info, [child_ptr]);
                match child_valuedata.kind() {
                    koopa::ir::ValueKind::Aggregate(_) => {
                        aggregate_to_store_insts(child, child_ptr, program, my_ir_generator_info)?;
                    }
                    _ => {
                        let store_inst = create_new_local_value(program, my_ir_generator_info)
                            .store(child, child_ptr);
                        insert_local_instructions(program, my_ir_generator_info, [store_inst]);
                    }
                }
            }
        }
        _ => panic!("This value must be an aggregate value! "),
    }

    Ok(())
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
                            "Non-constant expression '{:?}' in global variable initval: {:?}",
                            exp, self
                        ))
                    } else {
                        Ok(IRInitValBuildResult::Var(value))
                    }
                }
            },
            InitVal::Aggregate(childs) => {
                let (value, _) =
                    build_new_aggregate(shape, childs, is_global, program, my_ir_generator_info)?;
                Ok(IRInitValBuildResult::Aggregate(value))
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
                        "Non-constant expression in constant initval: {:?}",
                        const_def
                    ))
                }
                IRInitValBuildResult::Aggregate(aggr) => {
                    let array_ptr = if my_ir_generator_info.curr_func.is_some() {
                        let aggr_valuedata = get_valuedata(aggr, program, my_ir_generator_info);
                        let addr = create_new_local_value(program, my_ir_generator_info)
                            .alloc(aggr_valuedata.ty().clone());
                        insert_local_instructions(program, my_ir_generator_info, [addr]);
                        aggregate_to_store_insts(aggr, addr, program, my_ir_generator_info)?;
                        addr
                    } else {
                        let addr = program.new_value().global_alloc(aggr);
                        program.set_value_name(addr, Some(format!("@{}", ident.content,)));
                        addr
                    };
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
                    // Allocate the new local variable.
                    let var_addr = create_new_local_value(program, my_ir_generator_info)
                        .alloc(Type::get(var_type.clone()));
                    program
                        .func_mut(func)
                        .dfg_mut()
                        .set_value_name(var_addr, Some(format!("@{}", ident.content,)));
                    insert_local_instructions(program, my_ir_generator_info, [var_addr]);
                    // Build RHS value (if exists).
                    let rhs_result = if let Some(rhs) = possible_rhs {
                        let result = rhs.build(&shape, program, my_ir_generator_info)?;
                        match result {
                            IRInitValBuildResult::Const(int) => Some(
                                create_new_local_value(program, my_ir_generator_info).integer(int),
                            ),
                            IRInitValBuildResult::Var(value) => Some(value),
                            IRInitValBuildResult::Aggregate(value) => {
                                // Do not straightly store aggregate initval.
                                aggregate_to_store_insts(
                                    value,
                                    var_addr,
                                    program,
                                    my_ir_generator_info,
                                )?;
                                None
                            }
                        }
                    } else {
                        None
                    };
                    // Assign the RHS value into the new variable (if needed).
                    if let Some(rhs_value) = rhs_result {
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
                    // Allocate the new global variable.
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
