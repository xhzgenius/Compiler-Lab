//! This module is the backend of my compiler.
//! It converts the Koopa IR into assembly code.

mod build_assembly;
use std::collections::HashMap;
use std::io::Write;

use build_assembly::AssemblyBuildable;
use koopa::ir::{FunctionData, Program, Value};

pub fn generate_assembly(program: &Program, output_file: &mut std::fs::File) -> Result<(), String> {
    let codes = program.build(program)?;
    for code in codes {
        writeln!(output_file, "{}", code).expect("Write error. ");
    }
    Ok(())
}

const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "x31",
];

// const REGISTER_FOR_TEMP: [usize; 7] = [5, 6, 7, 28, 29, 30, 31];
const REGISTER_FOR_TEMP: [usize; 6] = [5, 6, 7, 28, 29, 30];
// const REGISTER_FOR_TEMP: [usize; 2] = [5, 6];

const REGISTER_FOR_ARGS: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];

const ARG_SIZE: usize = 4;

const REG_A0: usize = 10;

const REG_SP: usize = 2;

/// This register is for temporary integers only!
const REG_X31: usize = 31;

const MAX_SHORT_INT: isize = 2047;
const MIN_SHORT_INT: isize = -2048;

#[derive(Debug)]
pub struct FuncValueTable {
    curr_time: i32,
    register_user: [Option<Value>; 32],
    register_used_time: [i32; 32], // LRU registers
    value_location: HashMap<Value, ValueLocation>,
}

#[derive(Debug)]
pub enum ValueLocation {
    Local(usize),
    Global(String),
}

impl FuncValueTable {
    fn new() -> FuncValueTable {
        FuncValueTable {
            curr_time: 0,
            register_user: [None; 32],
            register_used_time: [0; 32],
            value_location: HashMap::new(),
        }
    }

    fn __update_user(&mut self, reg: usize, user: Value) {
        self.curr_time += 1;
        self.register_user[reg] = Some(user);
        self.register_used_time[reg] = self.curr_time;
    }
    fn __free_user(&mut self, reg: usize) {
        self.register_user[reg] = None;
        self.register_used_time[reg] = 0;
    }

    fn __is_value_in_register(&self, value: Value) -> Option<usize> {
        for i in 0..REGISTER_NAMES.len() {
            match self.register_user[i] {
                Some(v) => {
                    if v == value {
                        return Some(i);
                    }
                }
                None => {}
            }
        }
        None
    }

    fn assign_v1_to_v2(
        &mut self,
        v1: Value,
        v2: Value,
        program: &Program,
        fd: &FunctionData,
    ) -> Vec<String> {
        let mut codes = vec![];
        let (reg1, codes1) = self.want_to_visit_value(v1, program, fd, true, None);
        let (reg2, codes2) = self.want_to_visit_value(v2, program, fd, false, None);
        codes.extend(codes1);
        codes.extend(codes2);
        codes.push(format!(
            "  mv\t{}, {}",
            REGISTER_NAMES[reg2], REGISTER_NAMES[reg1]
        ));
        codes
    }

    fn store_with_offset(&mut self, reg: usize, offset: isize) -> Vec<String> {
        let mut codes = vec![];
        if MIN_SHORT_INT <= offset && offset <= MAX_SHORT_INT {
            codes.push(format!("  sw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
        } else {
            codes.push(format!(
                "  li\t{}, {}\n  add\t{}, {}, sp\n  sw\t{}, 0({})",
                REGISTER_NAMES[REG_X31],
                offset,
                REGISTER_NAMES[REG_X31],
                REGISTER_NAMES[REG_X31],
                REGISTER_NAMES[reg],
                REGISTER_NAMES[REG_X31]
            ));
        }
        codes
    }
    fn load_with_offset(&mut self, reg: usize, offset: isize) -> Vec<String> {
        let mut codes = vec![];
        if MIN_SHORT_INT <= offset && offset <= MAX_SHORT_INT {
            codes.push(format!("  lw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
        } else {
            codes.push(format!(
                "  li\t{}, {}\n  add\t{}, {}, sp\n  lw\t{}, 0({})",
                REGISTER_NAMES[REG_X31],
                offset,
                REGISTER_NAMES[REG_X31],
                REGISTER_NAMES[REG_X31],
                REGISTER_NAMES[reg],
                REGISTER_NAMES[REG_X31]
            ));
        }
        codes
    }
    fn add_with_offset(&mut self, reg: usize, offset: isize) -> Vec<String> {
        let mut codes = vec![];
        if MIN_SHORT_INT <= offset && offset <= MAX_SHORT_INT {
            codes.push(format!(
                "  addi\t{}, {}, {}",
                REGISTER_NAMES[reg], REGISTER_NAMES[reg], offset
            ));
        } else {
            codes.push(format!(
                "  li\t{}, {}\n  add\t{}, {}, {}",
                REGISTER_NAMES[REG_X31],
                offset,
                REGISTER_NAMES[reg],
                REGISTER_NAMES[reg],
                REGISTER_NAMES[REG_X31],
            ));
        }
        codes
    }
    fn load_global(&mut self, reg: usize, symbol_name: String) -> Vec<String> {
        vec![format!(
            "  la\t{}, {}\n  lw\t{}, 0({})",
            REGISTER_NAMES[REG_X31], symbol_name, REGISTER_NAMES[reg], REGISTER_NAMES[REG_X31]
        )]
    }
    fn store_global(&mut self, reg: usize, symbol_name: String) -> Vec<String> {
        vec![format!(
            "  la\t{}, {}\n  sw\t{}, 0({})",
            REGISTER_NAMES[REG_X31], symbol_name, REGISTER_NAMES[reg], REGISTER_NAMES[REG_X31]
        )]
    }

    /// Kick a value and store it to the stack.
    fn save_register(&mut self, reg: usize) -> Vec<String> {
        let kicked_value = match self.register_user[reg] {
            Some(value) => value,
            None => return vec![],
        };
        let mut codes = vec![];
        match self
            .value_location
            .get(&kicked_value)
            .expect("Can't find kicked value in table. Seems impossible. ")
        {
            ValueLocation::Local(offset) => {
                let offset = offset.clone();
                codes.extend(self.store_with_offset(reg, offset as isize));
            }
            ValueLocation::Global(symbol_name) => {
                let symbol_name = symbol_name.clone();
                codes.extend(self.store_global(reg, symbol_name));
            }
        }
        self.__free_user(reg);
        codes
    }

    /// Finds a usable register.
    /// If all registers are being used, then kicks one.
    fn get_tmp_reg(&mut self) -> (usize, Vec<String>) {
        self.curr_time += 1;
        let mut now_min = std::i32::MAX;
        let mut possible_choice: Option<usize> = None;
        for i in REGISTER_FOR_TEMP {
            match self.register_user[i] {
                Some(_) => {
                    if self.register_used_time[i] < now_min {
                        now_min = self.register_used_time[i];
                        possible_choice = Some(i);
                    }
                }
                None => return (i, vec![]),
            }
        }
        let choice = possible_choice.unwrap();
        (choice, self.save_register(choice))
    }

    /// Want to visit a value. Make it appear in a register.
    fn want_to_visit_value(
        &mut self,
        value: Value,
        program: &Program,
        fd: &FunctionData,
        do_load: bool,
        use_certain_reg: Option<usize>,
    ) -> (usize, Vec<String>) {
        self.curr_time += 1;
        let value_kind = match value.is_global() {
            true => program.borrow_value(value).kind().clone(),
            false => fd.dfg().value(value).kind().clone(),
        };
        if let koopa::ir::ValueKind::Integer(int) = value_kind {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            let (dst_reg, mut codes) = match use_certain_reg {
                Some(reg) => (reg, vec![]),
                None => self.get_tmp_reg(),
            };
            codes.push(format!("  li\t{}, {}", REGISTER_NAMES[dst_reg], int.value()));
            // self.__update_user(dst_reg, value);
            return (dst_reg, codes);
        }
        // Value already in a register
        if let Some(src_reg) = self.__is_value_in_register(value) {
            match use_certain_reg {
                Some(reg_dst) => {
                    if src_reg != reg_dst {
                        self.__update_user(reg_dst, value);
                        self.__free_user(src_reg);
                        return (
                            reg_dst,
                            vec![format!(
                                "  mv\t{}, {}",
                                REGISTER_NAMES[reg_dst], REGISTER_NAMES[src_reg]
                            )],
                        );
                    } else {
                        return (src_reg, vec![]);
                    }
                }
                None => return (src_reg, vec![]),
            };
        }
        // Value not in registers
        let (reg, mut codes) = match use_certain_reg {
            Some(reg_dst) => (reg_dst, vec![]),
            None => self.get_tmp_reg(),
        };
        if do_load {
            match self
                .value_location
                .get(&value)
                .expect("Can't find wanted-to-visit value in table. ")
            {
                ValueLocation::Local(offset) => {
                    let offset = offset.clone();
                    codes.extend(self.load_with_offset(reg, offset as isize));
                }
                ValueLocation::Global(symbol_name) => {
                    let symbol_name = symbol_name.clone();
                    codes.extend(self.load_global(reg, symbol_name));
                }
            }
        }
        self.__update_user(reg, value);
        (reg, codes)
    }

    // /// Frees a register.
    // fn free_register(&mut self, reg: usize) {
    //     self.register_user[reg] = None;
    //     self.register_used_time[reg] = 0;
    // }
}
