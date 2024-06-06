//! Convert a single Koopa IR component into assembly code.

use std::vec;

use super::{
    FuncValueTable, ValueLocation, ARG_SIZE, REGISTER_FOR_ARGS, REGISTER_FOR_TEMP, REGISTER_NAMES,
    REG_A0,
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
        let mut my_table = FuncValueTable::new();

        // In my compiler, every defined local variable (like "@y = alloc i32")
        // and temp values has its place in memory.
        // The registers work like a LRU cache.

        let mut max_call_arg_size = 0; // Bytes for storing all call args.
        let reg_ra_size = 4; // Bytes for storing register ra's value.
        for (&_block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                if let ValueKind::Call(call) = self.dfg().value(value).kind() {
                    let mut arg_size = 0;
                    if call.args().len() > REGISTER_FOR_ARGS.len() {
                        for &_arg in &call.args()[REGISTER_FOR_ARGS.len()..] {
                            arg_size += ARG_SIZE;
                        }
                    }
                    max_call_arg_size = std::cmp::max(arg_size, max_call_arg_size);
                }
            }
        }
        let mut local_var_size: usize = 0; // Bytes for storing all local variables.
        for (&_block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                my_table.value_location.insert(
                    value,
                    ValueLocation::Local(local_var_size + max_call_arg_size),
                );
                local_var_size += value_data.ty().size();
            }
        }
        let stack_frame_size = (local_var_size + max_call_arg_size + reg_ra_size + 15) / 16 * 16;

        // Change the stack pointer.
        if stack_frame_size <= 2048 {
            prologue_codes.push(format!("  addi\tsp, sp, -{}", stack_frame_size));
        } else {
            prologue_codes.push(format!(
                "  li\tt0, -{}\n  add\tsp, sp, t0",
                stack_frame_size
            ));
        }

        // Push every arg and global vars into the value table.
        for i in 0..std::cmp::min(REGISTER_FOR_ARGS.len(), self.params().len()) {
            my_table.register_user[REGISTER_FOR_ARGS[i]] = Some(self.params()[i]);
        }
        for i in REGISTER_FOR_ARGS.len()..self.params().len() {
            my_table.value_location.insert(
                self.params()[i],
                ValueLocation::Local((i - REGISTER_FOR_ARGS.len()) * ARG_SIZE + stack_frame_size),
            );
        }
        for &global in program.inst_layout() {
            my_table.value_location.insert(
                global,
                ValueLocation::Global(
                    program.borrow_value(global).name().clone().unwrap()[1..].to_string(),
                ),
            );
        }

        // Save callee-saved registers.
        // (Currently no callee-saved registers need to be saved. )

        // Save registar ra. (Return address)
        prologue_codes.push(format!("  sw\tra, {}(sp)", stack_frame_size - reg_ra_size));

        let mut body_codes = vec![];
        body_codes.push(format!("\n.{}_body:", &self.name()[1..]));

        // dbg!(&self.name(), &my_table);

        for (&block, node) in self.layout().bbs() {
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
                match value_data.kind() {
                    // Do different things based on instruction kind.

                    // Return instruction
                    koopa::ir::ValueKind::Return(return_inst) => {
                        // Does it have a return value?
                        match return_inst.value() {
                            Some(return_value) => {
                                let (reg, codes) = my_table.want_to_visit_value(
                                    return_value,
                                    program,
                                    self,
                                    true,
                                    Some(REG_A0),
                                );
                                assert_eq!(reg, REG_A0, "WTF??! I asked to load into reg a0!!!");
                                body_codes.extend(codes);
                            }
                            None => {}
                        }
                        // Jump to the return part.
                        body_codes.push(format!("  j\t.{}_ret", &self.name()[1..]));
                    }

                    // Binary operation
                    koopa::ir::ValueKind::Binary(binary) => {
                        let (reg1, codes1) =
                            my_table.want_to_visit_value(binary.lhs(), program, self, true, None);
                        let (reg2, codes2) =
                            my_table.want_to_visit_value(binary.rhs(), program, self, true, None);
                        body_codes.extend(codes1);
                        body_codes.extend(codes2);
                        let (reg_ans, codes) =
                            my_table.want_to_visit_value(value, program, self, false, None);
                        body_codes.extend(codes);
                        body_codes.push(binary_op_to_assembly(binary, reg_ans, reg1, reg2));

                        // If the result is useless, free the register.
                        // 我真是他妈天才！但是只天才了一半
                        // if self.dfg().value(value).used_by().is_empty() {
                        //     my_table.free_register(reg_ans);
                        // }
                    }

                    // Alloc operation
                    koopa::ir::ValueKind::Alloc(_) => {}

                    // Store operation
                    koopa::ir::ValueKind::Store(store) => {
                        body_codes.extend(my_table.assign_v1_to_v2(
                            store.value(),
                            store.dest(),
                            program,
                            self,
                        ));
                    }

                    // Load operation
                    koopa::ir::ValueKind::Load(load) => {
                        body_codes.extend(my_table.assign_v1_to_v2(
                            load.src(),
                            value,
                            program,
                            self,
                        ));
                    }

                    // Jump operation
                    koopa::ir::ValueKind::Jump(jump) => {
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
                            my_table.want_to_visit_value(branch.cond(), program, self, true, None);
                        body_codes.extend(codes);
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
                        // Save caller-saved registers.
                        for reg in REGISTER_FOR_TEMP {
                            body_codes.extend(my_table.save_register(reg));
                        }

                        // Push args into registers for args.
                        for i in 0..std::cmp::min(REGISTER_FOR_ARGS.len(), call.args().len()) {
                            let (reg, codes) = my_table.want_to_visit_value(
                                call.args()[i],
                                program,
                                self,
                                true,
                                Some(REGISTER_FOR_ARGS[i]),
                            );
                            assert_eq!(
                                reg, REGISTER_FOR_ARGS[i],
                                "WTF??! I asked to load into this reg!!!"
                            );
                            body_codes.extend(codes);
                        }
                        for i in REGISTER_FOR_ARGS.len()..call.args().len() {
                            let (reg, codes) = my_table.want_to_visit_value(
                                call.args()[i],
                                program,
                                self,
                                true,
                                None,
                            );
                            body_codes.extend(codes);
                            let offset = (i - REGISTER_FOR_ARGS.len()) * ARG_SIZE;
                            body_codes
                                .push(format!("  sw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
                        }

                        // Call the function.
                        body_codes.push(format!(
                            "  call\t{}",
                            &program.func(call.callee()).name()[1..]
                        ));

                        // Now the returned value is in register a0.
                        // Insert it into the register table.
                        my_table.register_user[REG_A0] = Some(value);
                        my_table.register_used_time[REG_A0] = my_table.curr_time;
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

        // Store back all the global variables in registers. 
        epilogue_codes.push(format!("# Save global variables."));
        for i in 0..REGISTER_NAMES.len() {
            if let Some(value) = my_table.register_user[i] {
                if value.is_global() {
                    epilogue_codes.extend(my_table.save_register(i));
                }
            }
        }

        // Restore registar ra. (Return address)
        epilogue_codes.push(format!("  lw\tra, {}(sp)", stack_frame_size - reg_ra_size));

        // Restore callee-saved registers.
        // (Currently no callee-saved registers need to be restored. )

        // Restore stack pointer.
        if stack_frame_size <= 2047 {
            epilogue_codes.push(format!("  addi\tsp, sp, {}", stack_frame_size));
        } else {
            epilogue_codes.push(format!("  li\tt0, {}\n  add\tsp, sp, t0", stack_frame_size));
        }

        // Return
        epilogue_codes.push(format!("  ret\n"));

        let mut all_codes = vec![];
        all_codes.extend(prologue_codes);
        all_codes.extend(body_codes);
        all_codes.extend(epilogue_codes);
        Ok(all_codes)
    }
}
