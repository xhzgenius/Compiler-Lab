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
            if int.value()==0 {
                Ok(0) // Register x0(id=0) is always 0. 
            }
            else {
                let reg = my_agi.allocate_register(value);
                writeln!(my_agi.output_file, "  li\t{}, {}", REGISTER_NAMES[reg], int.value()).expect("Write error. ");
                Ok(reg)
            }
        }
        koopa::ir::ValueKind::Binary(_) => {
            // Use the register allocated for this expression.
            let reg = my_agi.find_using_register(value).expect(
                format!(
                    "No register for expression. This should not happen. \n{:?}{:?}",
                    fd.dfg().value(value),
                    my_agi
                )
                .as_str(),
            );
            Ok(reg)
        }
        value_kind => Err(format!("Wrong type in LHS: {:?}", value_kind)),
    }
}

impl AssemblyBuildable for FunctionData {
    fn build(&self, my_agi: &mut MyAssemblyGeneratorInfo) -> Result<(), String> {
        writeln!(my_agi.output_file, "  .global {}", &self.name()[1..]).expect("Write error. ");
        writeln!(my_agi.output_file, "{}:", &self.name()[1..]).expect("Write error. ");

        // Save callee-saved registers. 
        // TODO

        // Clear register usages when entering the function. 
        my_agi.curr_time = 0;
        my_agi.register_used_time = [0; 32];
        my_agi.register_user = [None; 32];

        for (&_block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction.
                match value_data.kind() {
                    // DO different things based on instruction kind.

                    // Return instruction
                    koopa::ir::ValueKind::Return(return_inst) => {
                        match return_inst.value() {
                            // Does it have a return value?
                            Some(return_value) => {
                                let return_value_data = self.dfg().value(return_value);
                                match return_value_data.kind() {
                                    // Return value kind
                                    // Integer return value
                                    koopa::ir::ValueKind::Integer(int) => {
                                        writeln!(my_agi.output_file, "  li a0, {}", int.value())
                                            .expect("Write error. ");
                                    }
                                    koopa::ir::ValueKind::Binary(_) => {
                                        writeln!(my_agi.output_file, "  mv a0, {}", REGISTER_NAMES[my_agi.find_using_register(return_value).expect("No register for return value. Should never happen! ")]).expect("Write error. ");
                                    }
                                    // Other (TODO: not implemented)
                                    inst_kind => {
                                        return Err(format!(
                                            "Unknown Koopa IR instruction value {:?}",
                                            inst_kind
                                        ))
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
                        match binary.op() {
                            koopa::ir::BinaryOp::Add => {
                                writeln!(
                                    my_agi.output_file,
                                    "  add\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Sub => {
                                writeln!(
                                    my_agi.output_file,
                                    "  sub\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Mul => {
                                writeln!(
                                    my_agi.output_file,
                                    "  mul\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Div => {
                                writeln!(
                                    my_agi.output_file,
                                    "  div\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Mod => {
                                writeln!(
                                    my_agi.output_file,
                                    "  rem\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Eq => {
                                writeln!(
                                    my_agi.output_file,
                                    "  xor\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                                writeln!(
                                    my_agi.output_file,
                                    "  seqz\t{}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans],
                                )
                                .expect("Write error. "); // If a==b, then a^b==0.
                            }
                            koopa::ir::BinaryOp::NotEq => {
                                writeln!(
                                    my_agi.output_file,
                                    "  xor\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                                writeln!(
                                    my_agi.output_file,
                                    "  snez\t{}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans],
                                )
                                .expect("Write error. "); // If a==b, then a^b==0.
                            }
                            koopa::ir::BinaryOp::Lt => {
                                writeln!(
                                    my_agi.output_file,
                                    "  slt\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Gt => {
                                writeln!(
                                    my_agi.output_file,
                                    "  sgt\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Le => {
                                writeln!(
                                    my_agi.output_file,
                                    "  sgt\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                                writeln!(
                                    my_agi.output_file,
                                    "  seqz\t{}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans],
                                )
                                .expect("Write error. "); // LE is (not GT). 
                            }
                            koopa::ir::BinaryOp::Ge => {
                                writeln!(
                                    my_agi.output_file,
                                    "  slt\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                                writeln!(
                                    my_agi.output_file,
                                    "  seqz\t{}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg_ans],
                                )
                                .expect("Write error. "); // GE is (not LT). 
                            }
                            koopa::ir::BinaryOp::And => {
                                writeln!(
                                    my_agi.output_file,
                                    "  and\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Or => {
                                writeln!(
                                    my_agi.output_file,
                                    "  or\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Xor => {
                                writeln!(
                                    my_agi.output_file,
                                    "  xor\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Shl => {
                                writeln!(
                                    my_agi.output_file,
                                    "  sll\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Shr => {
                                writeln!(
                                    my_agi.output_file,
                                    "  srl\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                            koopa::ir::BinaryOp::Sar => {
                                writeln!(
                                    my_agi.output_file,
                                    "  sra\t{}, {}, {}",
                                    REGISTER_NAMES[reg_ans], REGISTER_NAMES[reg1], REGISTER_NAMES[reg2],
                                )
                                .expect("Write error. ");
                            }
                        }
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
