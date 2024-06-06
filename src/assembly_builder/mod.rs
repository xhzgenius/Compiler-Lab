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

const REGISTER_FOR_TEMP: [usize; 7] = [5, 6, 7, 28, 29, 30, 31];
// const REGISTER_FOR_TEMP: [usize; 2] = [5, 6];

const REGISTER_FOR_ARGS: [usize; 8] = [10, 11, 12, 13, 14, 15, 16, 17];

const ARG_SIZE: usize = 4;

const REG_A0: usize = 10;

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

    fn assign_v1_to_v2(&self, v1: Value, v2: Value) -> Vec<String> {
        todo!()
    }

    /// Kick a value and store it to the stack.
    fn save_register(&mut self, reg: usize) -> Vec<String> {
        let kicked_value = match self.register_user[reg] {
            Some(value) => value,
            None => return vec![],
        };
        self.register_user[reg] = None;
        self.register_used_time[reg] = 0;
        let mut codes = vec![];
        match self
            .value_location
            .get(&kicked_value)
            .expect("Can't find kicked value in table. Seems impossible. ")
        {
            ValueLocation::Local(offset) => {
                let offset = offset.clone();
                if offset <= 2047 {
                    codes.push(format!("  sw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
                } else {
                    let (tmp_reg, tmp_reg_codes) = self.get_tmp_reg();
                    codes.extend(tmp_reg_codes);
                    codes.push(format!(
                        "  li\t{}, {}\n  sw\t{}, 0({})",
                        REGISTER_NAMES[tmp_reg],
                        offset,
                        REGISTER_NAMES[reg],
                        REGISTER_NAMES[tmp_reg]
                    ));
                }
            }
            ValueLocation::Global(symbol_name) => {
                let symbol_name = symbol_name.clone();
                let (tmp_reg, tmp_codes) = self.get_tmp_reg();
                codes.extend(tmp_codes);
                codes.push(format!(
                    "  la\t{}, {}\n  sw\t{}, 0({})",
                    REGISTER_NAMES[tmp_reg],
                    symbol_name,
                    REGISTER_NAMES[reg],
                    REGISTER_NAMES[tmp_reg]
                ));
            }
        }
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
        let value_kind = match value.is_global() {
            true => program.borrow_value(value).kind().clone(),
            false => fd.dfg().value(value).kind().clone(),
        };
        if let koopa::ir::ValueKind::Integer(int) = value_kind {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            if int.value() == 0 {
                return match use_certain_reg {
                    Some(reg_dst) => (
                        reg_dst,
                        vec![format!("  li\t{}, 0", REGISTER_NAMES[reg_dst])],
                    ),
                    None => (0, vec![]), // Register x0(id=0) is always 0.
                };
            } else {
                // Allocate a new register for the constant integer.
                let (reg, mut codes) = match use_certain_reg {
                    Some(reg_dst) => (reg_dst, vec![]),
                    None => self.get_tmp_reg(),
                };
                codes.push(format!("  li\t{}, {}", REGISTER_NAMES[reg], int.value()));
                return (reg, codes);
            }
        }
        // Value already in a register
        if let Some(reg) = self.__is_value_in_register(value) {
            match use_certain_reg {
                Some(reg_dst) => {
                    if reg != reg_dst {
                        self.register_user[reg_dst] = Some(value);
                        self.register_used_time[reg_dst] = self.curr_time;
                        self.register_user[reg] = None;
                        self.register_used_time[reg] = 0;
                        return (
                            reg_dst,
                            vec![format!(
                                "  mv\t{}, {}",
                                REGISTER_NAMES[reg_dst], REGISTER_NAMES[reg]
                            )],
                        );
                    } else {
                        return (reg, vec![]);
                    }
                }
                None => return (reg, vec![]),
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
                    if offset <= 2047 {
                        codes.push(format!("  lw\t{}, {}(sp)", REGISTER_NAMES[reg], offset));
                    } else {
                        let (tmp_reg, tmp_reg_codes) = self.get_tmp_reg();
                        codes.extend(tmp_reg_codes);
                        codes.push(format!(
                            "  li\t{}, {}\n  lw\t{}, 0({})",
                            REGISTER_NAMES[tmp_reg],
                            offset,
                            REGISTER_NAMES[reg],
                            REGISTER_NAMES[tmp_reg]
                        ));
                    }
                }
                ValueLocation::Global(symbol_name) => {
                    let symbol_name = symbol_name.clone();
                    let (tmp_reg, tmp_codes) = self.get_tmp_reg();
                    codes.extend(tmp_codes);
                    codes.push(format!(
                        "  la\t{}, {}\n  lw\t{}, 0({})",
                        REGISTER_NAMES[tmp_reg],
                        symbol_name,
                        REGISTER_NAMES[reg],
                        REGISTER_NAMES[tmp_reg]
                    ));
                }
            }
        }
        self.register_user[reg] = Some(value);
        self.register_used_time[reg] = self.curr_time;
        (reg, codes)
    }

    // /// Frees a register.
    // fn free_register(&mut self, reg: usize) {
    //     self.register_user[reg] = None;
    //     self.register_used_time[reg] = 0;
    // }
}
