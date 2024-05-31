//! Convert a single Koopa IR component into assembly code.

use std::vec;

use super::{FuncValueTable, REGISTER_NAMES};
use koopa::ir::{FunctionData, Program};

pub trait AssemblyBuildable {
    fn build(&self) -> Result<Vec<String>, String>;
}

impl AssemblyBuildable for Program {
    fn build(&self) -> Result<Vec<String>, String> {
        // Assembly code of global variables
        // TODO

        // Assembly code of functions
        let mut function_codes = vec![];
        for &func in self.func_layout() {
            function_codes.extend(self.func(func).build()?);
        }
        Ok(function_codes)
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

impl AssemblyBuildable for FunctionData {
    fn build(&self) -> Result<Vec<String>, String> {
        let mut prologue_codes = vec![];
        prologue_codes.push(format!("  .text"));
        prologue_codes.push(format!("  .global {}", &self.name()[1..]));
        prologue_codes.push(format!("{}:", &self.name()[1..]));

        // Clear register usages when entering the function.
        let mut my_table = FuncValueTable::new();

        // In my compiler, every defined local variable (like "@y = alloc i32")
        // and temp values has its place in memory.
        // The registers work like a LRU cache.

        let mut local_var_size = 0; // Bytes for storing all local variables.

        for (&_block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                my_table.value_location.insert(value, local_var_size);
                local_var_size += value_data.ty().size();
            }
        }
        local_var_size = (local_var_size + 15) / 16 * 16;

        // Function prologue: change the stack pointer.
        if local_var_size <= 2048 {
            prologue_codes.push(format!("  addi\tsp, sp, -{}", local_var_size));
        } else {
            prologue_codes.push(format!("  li\tt0, -{}\n  add\tsp, sp, t0", local_var_size));
        }

        // Save callee-saved registers.
        // (Currently no callee-saved registers need to be saved. )

        let mut body_codes = vec![];
        body_codes.push(format!("\n{}_body:", &self.name()[1..]));

        dbg!(&self.name(), &my_table);

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
                body_codes.push(format!("\n{}:", &bb_name[1..]));
            }

            // Generate instructions.
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                match value_data.kind() {
                    // DO different things based on instruction kind.

                    // Return instruction
                    koopa::ir::ValueKind::Return(return_inst) => {
                        // Does it have a return value?
                        match return_inst.value() {
                            Some(return_value) => {
                                let (reg, codes) =
                                    my_table.want_to_visit_value(return_value, self, true);
                                body_codes.extend(codes);
                                body_codes.push(format!("  mv\ta0, {}", REGISTER_NAMES[reg]));
                            }
                            None => {}
                        }
                        // Jump to the return part.
                        body_codes.push(format!("  j\t{}_ret", &self.name()[1..]));
                    }

                    // Binary operation
                    koopa::ir::ValueKind::Binary(binary) => {
                        let (reg1, codes1) = my_table.want_to_visit_value(binary.lhs(), self, true);
                        let (reg2, codes2) = my_table.want_to_visit_value(binary.rhs(), self, true);
                        body_codes.extend(codes1);
                        body_codes.extend(codes2);
                        let (reg_ans, codes) = my_table.want_to_visit_value(value, self, false);
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
                        let (stored_reg, codes) =
                            my_table.want_to_visit_value(store.value(), self, true);
                        body_codes.extend(codes);
                        let offset = *my_table.value_location.get(&store.dest()).expect(
                            format!("Can't find store.dest() in stack. Should not happen. ",)
                                .as_str(),
                        );
                        if offset <= 2047 {
                            body_codes.push(format!(
                                "  sw\t{}, {}(sp)",
                                REGISTER_NAMES[stored_reg], offset
                            ));
                        } else {
                            let (tmp_reg, tmp_reg_codes) = my_table.get_tmp_reg();
                            body_codes.extend(tmp_reg_codes);
                            body_codes.push(format!(
                                "  li\t{}, {}\n  sw\t{}, 0({})",
                                REGISTER_NAMES[tmp_reg],
                                offset,
                                REGISTER_NAMES[stored_reg],
                                REGISTER_NAMES[tmp_reg]
                            ));
                            // my_table.free_register(tmp_reg);
                        }
                        // my_table.free_register(stored_reg);
                    }

                    // Load operation
                    koopa::ir::ValueKind::Load(load) => {
                        let (loaded_reg, codes) = my_table.want_to_visit_value(value, self, false);
                        body_codes.extend(codes);
                        let offset = *my_table.value_location.get(&load.src()).expect(
                            format!("Can't find load.src() in stack. Should not happen. ",)
                                .as_str(),
                        );
                        if offset <= 2047 {
                            body_codes.push(format!(
                                "  lw\t{}, {}(sp)",
                                REGISTER_NAMES[loaded_reg], offset
                            ));
                        } else {
                            let (tmp_reg, tmp_reg_codes) = my_table.get_tmp_reg();
                            body_codes.extend(tmp_reg_codes);
                            body_codes.push(format!(
                                "  li\t{}, {}\n  lw\t{}, 0({})",
                                REGISTER_NAMES[tmp_reg],
                                offset,
                                REGISTER_NAMES[loaded_reg],
                                REGISTER_NAMES[tmp_reg]
                            ));
                            // my_table.free_register(tmp_reg);
                        }
                    }

                    // Jump operation
                    koopa::ir::ValueKind::Jump(jump) => {
                        body_codes.push(format!(
                            "  j\t{}",
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
                            my_table.want_to_visit_value(branch.cond(), self, true);
                        body_codes.extend(codes);
                        body_codes.push(format!(
                            "  bnez\t{}, {}",
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
                            "  j\t{}",
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

        // Restore callee-saved registers.
        // (Currently no callee-saved registers need to be restored. )

        // Function epilogue: change the stack pointer.
        let mut epilogue_codes = vec![];
        epilogue_codes.push(format!("\n{}_ret:", &self.name()[1..]));

        if local_var_size <= 2047 {
            epilogue_codes.push(format!("  addi\tsp, sp, {}", local_var_size));
        } else {
            epilogue_codes.push(format!("  li\tt0, {}\n  add\tsp, sp, t0", local_var_size));
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
