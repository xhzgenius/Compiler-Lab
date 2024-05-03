//! Convert a single Koopa IR component into assembly code.

use std::io::Write;

use super::{MyAssemblyGeneratorInfo, REGISTER_NAMES};
use koopa::ir::{FunctionData, Program, Value};

pub trait AssemblyBuildable {
    fn build(&self, my_agi: &mut MyAssemblyGeneratorInfo) -> Result<(), String>;
}

impl AssemblyBuildable for Program {
    fn build(&self, my_agi: &mut MyAssemblyGeneratorInfo) -> Result<(), String> {
        // Assembly code of global variables
        // TODO

        // Assembly code of functions
        writeln!(my_agi.output_file, "  .text").expect("Write error. ");
        for &func in self.func_layout() {
            self.func(func).build(my_agi)?;
        }
        Ok(())
    }
}

/// Given a value, find which register it should use.
fn get_reg(
    fd: &FunctionData,
    value: Value,
    my_agi: &mut MyAssemblyGeneratorInfo,
) -> Result<usize, String> {
    match fd.dfg().value(value).kind() {
        koopa::ir::ValueKind::Integer(int) => {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            if int.value() == 0 {
                Ok(0) // Register x0(id=0) is always 0.
            } else {
                // Allocate a new register for the constant integer.
                let reg = my_agi.allocate_register(value);
                writeln!(
                    my_agi.output_file,
                    "  li\t{}, {}",
                    REGISTER_NAMES[reg],
                    int.value()
                )
                .expect("Write error. ");
                Ok(reg)
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
            let reg = my_agi.find_using_register(value);
            Ok(reg)
        }
        // value_kind => Err(format!(
        //     "Wrong type, can't find register for it: {:?}",
        //     value_kind
        // )),
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
    fn build(&self, my_agi: &mut MyAssemblyGeneratorInfo) -> Result<(), String> {
        writeln!(my_agi.output_file, "  .global {}", &self.name()[1..]).expect("Write error. ");
        writeln!(my_agi.output_file, "{}:", &self.name()[1..]).expect("Write error. ");

        // Save callee-saved registers.
        // (Currently no callee-saved registers need to be saved. )

        // Clear register usages when entering the function.
        my_agi.curr_time = 0;
        my_agi.register_used_time = [0; 32];
        my_agi.register_user = [None; 32];

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
            writeln!(my_agi.output_file, "  addi sp, sp, -{}", local_var_size)
                .expect("Write error. ");
        } else {
            writeln!(
                my_agi.output_file,
                "  li t0, -{}\n  add sp, sp, t0",
                local_var_size
            )
            .expect("Write error. ");
        }

        for (&_block, node) in self.layout().bbs() {
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
                                        writeln!(my_agi.output_file, "  li a0, {}", int.value())
                                            .expect("Write error. ");
                                    }
                                    // Other return values (result of binary expressions or left values)
                                    _ => {
                                        writeln!(
                                            my_agi.output_file,
                                            "  mv a0, {}",
                                            REGISTER_NAMES
                                                [my_agi.find_using_register(return_value)]
                                        )
                                        .expect("Write error. ");
                                    }
                                }
                            }
                            None => {}
                        }
                        writeln!(my_agi.output_file, "  ret").expect("Write error. ");
                    }

                    // Binary operation
                    koopa::ir::ValueKind::Binary(binary) => {
                        let reg1 = get_reg(self, binary.lhs(), my_agi)?;
                        let reg2 = get_reg(self, binary.rhs(), my_agi)?;
                        my_agi.free_register(reg1);
                        my_agi.free_register(reg2);
                        // Allocate a register for result. It can overwrite lhs or rhs.
                        let reg_ans = my_agi.allocate_register(value);
                        writeln!(
                            my_agi.output_file,
                            "{}",
                            binary_op_to_assembly(binary, reg_ans, reg1, reg2)
                        )
                        .expect("Write error. ");
                    }

                    // Alloc operation
                    koopa::ir::ValueKind::Alloc(_) => {}

                    // Store operation
                    koopa::ir::ValueKind::Store(store) => {
                        let stored_reg = get_reg(self, store.value(), my_agi)?;
                        let local_var_name = self
                            .dfg()
                            .value(store.dest())
                            .name()
                            .clone()
                            .expect("Store target has no name. Should not happen. ");
                        writeln!(
                            my_agi.output_file,
                            "  sw {}, {}(sp)",
                            REGISTER_NAMES[stored_reg],
                            my_agi
                                .local_var_location_in_stack
                                .get(&local_var_name)
                                .expect(
                                    format!(
                                        "Can't find {} in stack. Should not happen. ",
                                        &local_var_name
                                    )
                                    .as_str()
                                )
                        )
                        .expect("Write error. ");
                    }

                    // Load operation
                    koopa::ir::ValueKind::Load(load) => {
                        let loaded_reg = my_agi.allocate_register(value);
                        let local_var_name = self
                            .dfg()
                            .value(load.src())
                            .name()
                            .clone()
                            .expect("Store target has no name. Should not happen. ");
                        writeln!(
                            my_agi.output_file,
                            "  lw {}, {}(sp)",
                            REGISTER_NAMES[loaded_reg],
                            my_agi
                                .local_var_location_in_stack
                                .get(&local_var_name)
                                .expect(
                                    format!(
                                        "Can't find {} in stack. Should not happen. ",
                                        &local_var_name
                                    )
                                    .as_str()
                                )
                        )
                        .expect("Write error. ");
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
        Ok(())
    }
}
