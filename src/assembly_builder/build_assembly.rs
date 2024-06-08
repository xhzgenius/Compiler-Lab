//! Convert a single Koopa IR component into assembly code.

use std::vec;

use super::{
    MyBBValueTable, ARG_SIZE, REGISTER_FOR_ARGS, REGISTER_FOR_TEMP, REGISTER_NAMES, REG_A0, REG_SP,
};
use koopa::ir::{entities::ValueData, FunctionData, Program, ValueKind};

pub trait AssemblyBuildable {
    fn build(&self, program: &Program) -> Result<Vec<String>, String>;
}

impl AssemblyBuildable for Program {
    fn build(&self, _: &Program) -> Result<Vec<String>, String> {
        let mut program_codes = vec![];

        // Assembly code of global variables
        program_codes.push(format!("  .data"));
        for &global in self.inst_layout() {
            program_codes.extend(self.borrow_value(global).build(self)?);
        }

        // Assembly code of functions
        program_codes.push(format!("  .text"));
        for &func in self.func_layout() {
            if self.func(func).layout().bbs().len() > 0 {
                program_codes.extend(self.func(func).build(self)?);
            }
        }
        Ok(program_codes)
    }
}

fn binary_op_to_assembly(
    binary: &koopa::ir::values::Binary,
    reg_ans: usize,
    reg1: usize,
    reg2: usize,
) -> String {
    match binary.op() {
        koopa::ir::BinaryOp::Add => {
            format!(
                "  add\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Sub => {
            format!(
                "  sub\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Mul => {
            format!(
                "  mul\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Div => {
            format!(
                "  div\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Mod => {
            format!(
                "  rem\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Eq => {
            format!(
                "  xor\t{}, {}, {}\n  seqz\t{}, {}",
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg1],
                REGISTER_NAMES[reg2],
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg_ans],
            ) // If a==b, then a^b==0.
        }
        koopa::ir::BinaryOp::NotEq => {
            format!(
                "  xor\t{}, {}, {}\n  snez\t{}, {}",
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg1],
                REGISTER_NAMES[reg2],
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg_ans],
            )
        }
        koopa::ir::BinaryOp::Lt => {
            format!(
                "  slt\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Gt => {
            format!(
                "  sgt\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Le => {
            format!(
                "  sgt\t{}, {}, {}\n  seqz\t{}, {}",
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg1],
                REGISTER_NAMES[reg2],
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg_ans],
            ) // LE is (not GT).
        }
        koopa::ir::BinaryOp::Ge => {
            format!(
                "  slt\t{}, {}, {}\n  seqz\t{}, {}",
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg1],
                REGISTER_NAMES[reg2],
                REGISTER_NAMES[reg_ans],
                REGISTER_NAMES[reg_ans],
            ) // GE is (not LT).
        }
        koopa::ir::BinaryOp::And => {
            format!(
                "  and\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Or => {
            format!(
                "  or\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Xor => {
            format!(
                "  xor\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Shl => {
            format!(
                "  sll\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Shr => {
            format!(
                "  srl\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
        koopa::ir::BinaryOp::Sar => {
            format!(
                "  sra\t{}, {}, {}",
                REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
            )
        }
    }
}

impl AssemblyBuildable for ValueData {
    /// Used to handle global variable declarations.
    /// The ValueData's kind should be GlobalAlloc. Or it will panic.
    fn build(&self, program: &Program) -> Result<Vec<String>, String> {
        if let ValueKind::GlobalAlloc(global) = self.kind() {
            let mut codes = vec![];
            codes.push(format!("{}:", &self.name().clone().unwrap()[1..]));
            let init_value_data = program.borrow_value(global.init());
            match init_value_data.kind() {
                ValueKind::Integer(int) => {
                    codes.push(format!("  .word {}\n", int.value()));
                }
                ValueKind::ZeroInit(_) => {
                    codes.push(format!("  .zero {}\n", init_value_data.ty().size()));
                }
                value_kind => panic!(
                    "Global variable {} has wrong kind of initialization: {:?}! ",
                    &self.name().clone().unwrap()[1..],
                    value_kind
                ),
            }
            Ok(codes)
        } else {
            panic!("Not a global alloc instruction. ")
        }
    }
}

impl AssemblyBuildable for FunctionData {
    fn build(&self, program: &Program) -> Result<Vec<String>, String> {
        let mut prologue_codes = vec![];
        prologue_codes.push(format!("  .global {}", &self.name()[1..]));
        prologue_codes.push(format!("{}:", &self.name()[1..]));

        // Clear register usages when entering the function.
        let mut my_table = MyBBValueTable::new(program, self);

        // In my compiler, every defined local variable (like "@y = alloc i32")
        // and temp values has its place in memory.
        // The registers work like a LRU cache.

        // Calculate the stack frame size.
        let reg_ra_size = 4; // Bytes for storing register ra's value.
        let mut local_var_size = 0; // Bytes for storing local variables.
        for &value in self.dfg().values().keys() {
            if value.is_global() || my_table.is_temp_value(value) {
                continue;
            }
            local_var_size += self.dfg().value(value).ty().size();
        }
        let mut max_call_arg_size = 0; // Bytes for storing all call args.
        let mut max_temp_var_size = 0; // Bytes for storing temp values.
        for (&_block, node) in self.layout().bbs() {
            let mut temp_var_size = 0;
            for &value in node.insts().keys() {
                if !my_table.is_temp_value(value) {
                    continue;
                }
                let value_data = self.dfg().value(value);
                if let ValueKind::Call(call) = value_data.kind() {
                    let mut arg_size = 0;
                    if call.args().len() > REGISTER_FOR_ARGS.len() {
                        for &_arg in &call.args()[REGISTER_FOR_ARGS.len()..] {
                            arg_size += ARG_SIZE;
                        }
                    }
                    max_call_arg_size = std::cmp::max(arg_size, max_call_arg_size);
                }
                temp_var_size += value_data.ty().size();
            }
            max_temp_var_size = std::cmp::max(max_temp_var_size, temp_var_size);
        }

        let stack_frame_size =
            (reg_ra_size + max_temp_var_size + local_var_size + max_call_arg_size + 15) / 16 * 16;

        // Change the stack pointer.
        prologue_codes.extend(my_table.add_with_offset(REG_SP, -(stack_frame_size as isize)));

        // Push every arg's location into the value table.
        for i in 0..std::cmp::min(REGISTER_FOR_ARGS.len(), self.params().len()) {
            my_table.register_user[REGISTER_FOR_ARGS[i]] = Some(self.params()[i]);
        }
        for i in REGISTER_FOR_ARGS.len()..self.params().len() {
            my_table.local_value_location.insert(
                self.params()[i],
                (i - REGISTER_FOR_ARGS.len()) * ARG_SIZE + stack_frame_size,
            );
        }

        // Push every local variable into the value table.
        let mut curr_offset = max_call_arg_size;
        for &value in self.dfg().values().keys() {
            if value.is_global() || my_table.is_temp_value(value) {
                continue;
            }
            my_table.local_value_location.insert(value, curr_offset);
            curr_offset += self.dfg().value(value).ty().size();
        }
        assert_eq!(
            curr_offset,
            max_call_arg_size + local_var_size,
            "Inconsistent offset!"
        );

        // Save callee-saved registers.
        // (Currently no callee-saved registers need to be saved. )

        // Save registar ra. (Return address)
        prologue_codes.push(format!("  sw\tra, {}(sp)", stack_frame_size - reg_ra_size));

        let mut body_codes = vec![];
        body_codes.push(format!("\n.{}_body:", &self.name()[1..]));

        // dbg!(&self.name(), &my_table);

        for (&block, node) in self.layout().bbs() {
            // Insert every temp values into the value table.
            let mut curr_offset = max_call_arg_size + local_var_size;
            for &value in node.insts().keys() {
                if !my_table.is_temp_value(value) {
                    continue;
                }
                my_table.local_value_location.insert(value, curr_offset);
                curr_offset += self.dfg().value(value).ty().size();
            }
            assert!(
                curr_offset <= max_call_arg_size + local_var_size + max_temp_var_size,
                "Inconsistent offset!"
            );

            // At the beginning of the BasicBlock, declare its name.
            let possible_bb_name = self
                .dfg()
                .bbs()
                .get(&block)
                .expect("Can't find BasicBlock. Should not happen. ")
                .name()
                .clone();
            if let Some(bb_name) = possible_bb_name {
                body_codes.push(format!("\n.{}:", &bb_name[1..]));
            }

            // Generate instructions.
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                body_codes.push(format!("# {:?}", value_data.kind()));
                // dbg!(value_data);
                match value_data.kind() {
                    // Do different things based on instruction kind.

                    // Return instruction
                    koopa::ir::ValueKind::Return(return_inst) => {
                        // Does it have a return value?
                        match return_inst.value() {
                            Some(return_value) => {
                                let (reg, codes) =
                                    my_table.want_to_visit_value(return_value, true, Some(REG_A0));
                                assert_eq!(reg, REG_A0, "WTF??! I asked to load into reg a0!!!");
                                body_codes.extend(codes);
                                my_table.remove_temp_value(return_value);
                            }
                            None => {}
                        }
                        // At the end of the basic block, store all global and local variables into memory.
                        body_codes.extend(my_table.store_global_variables());
                        body_codes.extend(my_table.store_local_variables());
                        // Jump to the return part.
                        body_codes.push(format!("  j\t.{}_ret", &self.name()[1..]));
                    }

                    // Binary operation
                    koopa::ir::ValueKind::Binary(binary) => {
                        let (reg1, codes1) = my_table.want_to_visit_value(binary.lhs(), true, None);
                        let (reg2, codes2) = my_table.want_to_visit_value(binary.rhs(), true, None);
                        body_codes.extend(codes1);
                        body_codes.extend(codes2);
                        my_table.remove_temp_value(binary.lhs());
                        my_table.remove_temp_value(binary.rhs());
                        let (reg_ans, codes) = my_table.want_to_visit_value(value, false, None);
                        body_codes.extend(codes);
                        body_codes.push(binary_op_to_assembly(binary, reg_ans, reg1, reg2));
                    }

                    // Alloc operation
                    koopa::ir::ValueKind::Alloc(_) => {}

                    // Store operation
                    koopa::ir::ValueKind::Store(store) => {
                        body_codes.extend(my_table.assign_v1_to_v2(store.value(), store.dest()));
                        my_table.remove_temp_value(store.value());
                    }

                    // Load operation
                    koopa::ir::ValueKind::Load(load) => {
                        body_codes.extend(my_table.assign_v1_to_v2(load.src(), value));
                    }

                    // Jump operation
                    koopa::ir::ValueKind::Jump(jump) => {
                        // At the end of the basic block, store all global and local variables into memory.
                        body_codes.extend(my_table.store_global_variables());
                        body_codes.extend(my_table.store_local_variables());
                        body_codes.push(format!(
                            "  j\t.{}",
                            &self
                                .dfg()
                                .bbs()
                                .get(&jump.target())
                                .expect("Can't find jump target BasicBlock. Should not happen. ")
                                .name()
                                .clone()
                                .expect("Jump target BasicBlock has no name. Should not happen. ")
                                [1..]
                        ));
                    }

                    // Branch operation
                    koopa::ir::ValueKind::Branch(branch) => {
                        let (cond_reg, codes) =
                            my_table.want_to_visit_value(branch.cond(), true, None);
                        body_codes.extend(codes);
                        my_table.remove_temp_value(branch.cond());
                        // At the end of the basic block, store all global and local variables into memory.
                        body_codes.extend(my_table.store_global_variables());
                        body_codes.extend(my_table.store_local_variables());
                        body_codes.push(format!(
                            "  bnez\t{}, .{}",
                            REGISTER_NAMES[cond_reg],
                            &self
                                .dfg()
                                .bbs()
                                .get(&branch.true_bb())
                                .expect("Can't find branch BasicBlock 1. Should not happen. ")
                                .name()
                                .clone()
                                .expect("Branch BasicBlock 1 has no name. Should not happen. ")
                                [1..]
                        ));
                        body_codes.push(format!(
                            "  j\t.{}",
                            &self
                                .dfg()
                                .bbs()
                                .get(&branch.false_bb())
                                .expect("Can't find branch BasicBlock 2. Should not happen. ")
                                .name()
                                .clone()
                                .expect("Branch BasicBlock 2 has no name. Should not happen. ")
                                [1..]
                        ));
                    }

                    koopa::ir::ValueKind::Call(call) => {
                        // Push args into registers for args.
                        for i in 0..std::cmp::min(REGISTER_FOR_ARGS.len(), call.args().len()) {
                            let (reg, codes) = my_table.want_to_visit_value(
                                call.args()[i],
                                true,
                                Some(REGISTER_FOR_ARGS[i]),
                            );
                            assert_eq!(
                                reg, REGISTER_FOR_ARGS[i],
                                "WTF??! I asked to load into this reg!!!"
                            );
                            body_codes.extend(codes);
                            my_table.remove_temp_value(call.args()[i]);
                        }
                        for i in REGISTER_FOR_ARGS.len()..call.args().len() {
                            let (reg, codes) =
                                my_table.want_to_visit_value(call.args()[i], true, None);
                            body_codes.extend(codes);
                            let offset = (i - REGISTER_FOR_ARGS.len()) * ARG_SIZE;
                            body_codes
                                .push(format!("  sw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
                            my_table.remove_temp_value(call.args()[i]);
                        }

                        // Save caller-saved registers.
                        for reg in REGISTER_FOR_TEMP {
                            body_codes.extend(my_table.save_register(reg));
                        }

                        // Store back all the global variables in registers.
                        body_codes.extend(my_table.store_global_variables());

                        // Call the function.
                        body_codes.push(format!(
                            "  call\t{}",
                            &program.func(call.callee()).name()[1..]
                        ));

                        // Now the returned value is in register a0.
                        let (reg, codes) = my_table.want_to_visit_value(value, false, None);
                        body_codes.push(format!("  mv\t{}, a0", REGISTER_NAMES[reg]));
                        body_codes.extend(codes);
                    }

                    // Other instructions (TODO: Not implemented)
                    value_kind => {
                        return Err(format!(
                            "Unknown Koopa IR instruction value {:?}",
                            value_kind
                        ))
                    }
                }
            }
        }

        // Function epilogue.
        let mut epilogue_codes = vec![];
        epilogue_codes.push(format!("\n.{}_ret:", &self.name()[1..]));

        // Restore registar ra. (Return address)
        epilogue_codes.push(format!("  lw\tra, {}(sp)", stack_frame_size - reg_ra_size));

        // Restore callee-saved registers.
        // (Currently no callee-saved registers need to be restored. )

        // Restore stack pointer.

        epilogue_codes.extend(my_table.add_with_offset(REG_SP, stack_frame_size as isize));

        // Return
        epilogue_codes.push(format!("  ret\n"));

        let mut all_codes = vec![];
        all_codes.extend(prologue_codes);
        all_codes.extend(body_codes);
        all_codes.extend(epilogue_codes);
        Ok(all_codes)
    }
}
