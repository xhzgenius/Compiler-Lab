//! Convert a single Koopa IR component into assembly code.

use std::vec;

use super::{MyAssemblyGeneratorInfo, REGISTER_NAMES};
use koopa::ir::{FunctionData, Program, Value};

pub trait AssemblyBuildable {
    fn build(&self) -> Result<Vec<String>, String>;
}

impl AssemblyBuildable for Program {
    fn build(&self) -> Result<Vec<String>, String> {
        // Assembly code of global variables
        // TODO

        // Assembly code of functions
        let mut function_codes = vec![];
        function_codes.push(format!("  .text"));
        for &func in self.func_layout() {
            function_codes.extend(self.func(func).build()?);
        }
        Ok(function_codes)
    }
}

/// Given a value, find which register it should use. Also returns the loading instruction code(s).
fn get_reg(
    fd: &FunctionData,
    value: Value,
    my_agi: &mut MyAssemblyGeneratorInfo,
) -> Result<(usize, Vec<String>), String> {
    match fd.dfg().value(value).kind() {
        koopa::ir::ValueKind::Integer(int) => {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            if int.value() == 0 {
                Ok((0, vec![])) // Register x0(id=0) is always 0.
            } else {
                // Allocate a new register for the constant integer.
                let reg = my_agi.allocate_register(value)?;
                Ok((
                    reg,
                    vec![format!("  li\t{}, {}", REGISTER_NAMES[reg], int.value())],
                ))
            }
        }
        _ => {
            // Use the register allocated for this expression or left value.
            // A register should be allocated.
            // let reg = my_agi.find_using_register(value).expect(
            //     format!(
            //         "No register for expression. This should not happen. \n{:?}{:?}",
            //         fd.dfg().value(value),
            //         my_agi
            //     )
            //     .as_str(),
            // );
            Ok((my_agi.find_using_register(value)?, vec![]))
        }
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
        prologue_codes.push(format!("  .global {}", &self.name()[1..]));
        prologue_codes.push(format!("{}:", &self.name()[1..]));

        // Clear register usages when entering the function.
        let mut my_agi = MyAssemblyGeneratorInfo::new();

        // In my compiler, every defined local variable (like "@y = alloc i32") has its place in memory.
        // Temporary values are stored in registers.
        // If registers are not enough, redundant temp values will be stored in memory.
        // It works like a LRU cache.

        let mut local_var_size = 0; // Bytes for storing all local variables.

        for (&_block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                if let koopa::ir::ValueKind::Alloc(_) = value_data.kind() {
                    let var_name = value_data
                        .name()
                        .clone()
                        .expect("Local variable has no name. Should not happen. ");
                    my_agi
                        .local_var_location_in_stack
                        .insert(var_name, local_var_size);
                    local_var_size += value_data.ty().size();
                }
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
                                let return_value_data = self.dfg().value(return_value);
                                // Return value kind:
                                match return_value_data.kind() {
                                    // Integer return value
                                    koopa::ir::ValueKind::Integer(int) => {
                                        body_codes.push(format!("  li\ta0, {}", int.value()));
                                    }
                                    // Other return values (result of binary expressions or left values)
                                    _ => {
                                        body_codes.push(format!(
                                            "  mv\ta0, {}",
                                            REGISTER_NAMES
                                                [my_agi.find_using_register(return_value)?]
                                        ));
                                    }
                                }
                            }
                            None => {}
                        }
                        // Jump to the return part.
                        body_codes.push(format!("  j\t{}_ret", &self.name()[1..]));
                    }

                    // Binary operation
                    koopa::ir::ValueKind::Binary(binary) => {
                        let (reg1, codes1) = get_reg(self, binary.lhs(), &mut my_agi)?;
                        let (reg2, codes2) = get_reg(self, binary.rhs(), &mut my_agi)?;
                        body_codes.extend(codes1);
                        body_codes.extend(codes2);
                        my_agi.free_register(reg1);
                        my_agi.free_register(reg2);
                        // Allocate a register for result. It can overwrite lhs or rhs.
                        let reg_ans = my_agi.allocate_register(value)?;

                        body_codes.push(binary_op_to_assembly(binary, reg_ans, reg1, reg2));

                        // If the result is useless, free the register.
                        // 我真是他妈天才！但是只天才了一半
                        if self.dfg().value(value).used_by().is_empty() {
                            my_agi.free_register(reg_ans);
                        }
                    }

                    // Alloc operation
                    koopa::ir::ValueKind::Alloc(_) => {}

                    // Store operation
                    koopa::ir::ValueKind::Store(store) => {
                        let (stored_reg, codes) = get_reg(self, store.value(), &mut my_agi)?;
                        body_codes.extend(codes);
                        let local_var_name = self
                            .dfg()
                            .value(store.dest())
                            .name()
                            .clone()
                            .expect("Store target has no name. Should not happen. ");
                        let offset = *my_agi
                            .local_var_location_in_stack
                            .get(&local_var_name)
                            .expect(
                                format!(
                                    "Can't find {} in stack. Should not happen. ",
                                    &local_var_name
                                )
                                .as_str(),
                            );
                        if offset <= 2047 {
                            body_codes.push(format!(
                                "  sw\t{}, {}(sp)",
                                REGISTER_NAMES[stored_reg], offset
                            ));
                        } else {
                            let tmp_reg = my_agi.allocate_register(value)?;
                            body_codes.push(format!(
                                "  li\t{}, {}\n  sw\t{}, 0({})",
                                REGISTER_NAMES[tmp_reg],
                                offset,
                                REGISTER_NAMES[stored_reg],
                                REGISTER_NAMES[tmp_reg]
                            ));
                            my_agi.free_register(tmp_reg);
                        }
                        my_agi.free_register(stored_reg);
                    }

                    // Load operation
                    koopa::ir::ValueKind::Load(load) => {
                        let loaded_reg = my_agi.allocate_register(value)?;
                        let local_var_name = self
                            .dfg()
                            .value(load.src())
                            .name()
                            .clone()
                            .expect("Load source has no name. Should not happen. ");
                        let offset = *my_agi
                            .local_var_location_in_stack
                            .get(&local_var_name)
                            .expect(
                                format!(
                                    "Can't find {} in stack. Should not happen. ",
                                    &local_var_name
                                )
                                .as_str(),
                            );
                        if offset <= 2047 {
                            body_codes.push(format!(
                                "  lw\t{}, {}(sp)",
                                REGISTER_NAMES[loaded_reg], offset
                            ));
                        } else {
                            let tmp_reg = my_agi.allocate_register(value)?;
                            body_codes.push(format!(
                                "  li\t{}, {}\n  lw\t{}, 0({})",
                                REGISTER_NAMES[tmp_reg],
                                offset,
                                REGISTER_NAMES[loaded_reg],
                                REGISTER_NAMES[tmp_reg]
                            ));
                            my_agi.free_register(tmp_reg);
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
                        let (cond_reg, codes) = get_reg(self, branch.cond(), &mut my_agi)?;
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
        epilogue_codes.push(format!("  ret"));

        let mut all_codes = vec![];
        all_codes.extend(prologue_codes);
        all_codes.extend(body_codes);
        all_codes.extend(epilogue_codes);
        Ok(all_codes)
    }
}
