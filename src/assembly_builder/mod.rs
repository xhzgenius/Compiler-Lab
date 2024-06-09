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
const REG_RA: usize = 1;
/// This register is for temporary integers only!
const REG_X31: usize = 31;

const MAX_SHORT_INT: isize = 2047;
const MIN_SHORT_INT: isize = -2048;

pub struct MyBBValueTable<'a> {
    program: &'a Program,
    fd: &'a FunctionData,
    curr_time: i32,
    register_user: [Option<Value>; 32],
    register_used_time: [i32; 32], // LRU registers
    local_value_location: HashMap<Value, usize>,
}

impl MyBBValueTable<'_> {
    fn new<'a>(program: &'a Program, fd: &'a FunctionData) -> MyBBValueTable<'a> {
        MyBBValueTable {
            program,
            fd,
            curr_time: 0,
            register_user: [None; 32],
            register_used_time: [0; 32],
            local_value_location: HashMap::new(),
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

    fn is_temp_value(&self, value: Value) -> bool {
        let value_data = match value.is_global() {
            true => self.program.borrow_value(value).clone(),
            false => self.fd.dfg().value(value).clone(),
        };
        match value_data.name() {
            Some(name) => name.starts_with("%"),
            None => true,
        }
    }

    fn remove_temp_value(&mut self, value: Value) {
        // assert!(
        //     self.is_temp_value(value),
        //     "Can only remove temp values in table!"
        // );
        if !self.is_temp_value(value) {
            return;
        }
        if let Some(reg) = self.__is_value_in_register(value) {
            self.__free_user(reg);
        }
        self.local_value_location.remove(&value);
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

    fn store_global_variables(&mut self) -> Vec<String> {
        let mut codes = vec![format!("# Save global variables.")];
        for i in 0..REGISTER_NAMES.len() {
            if let Some(value) = self.register_user[i] {
                if value.is_global() {
                    codes.extend(self.save_register(i));
                }
            }
        }
        codes
    }

    fn store_local_variables(&mut self) -> Vec<String> {
        let mut codes = vec![format!("# Save local variables.")];
        for i in 0..REGISTER_NAMES.len() {
            if let Some(value) = self.register_user[i] {
                if value.is_global() || self.is_temp_value(value) {
                    continue;
                }
                codes.extend(self.save_register(i));
            }
        }
        codes
    }

    /// Kick the value in a register and store it to memory.
    fn save_register(&mut self, reg: usize) -> Vec<String> {
        let kicked_value = match self.register_user[reg] {
            Some(value) => value,
            None => return vec![],
        };
        let value_data = match kicked_value.is_global() {
            true => self.program.borrow_value(kicked_value).clone(),
            false => self.fd.dfg().value(kicked_value).clone(),
        };
        let mut codes = vec![];
        // Store the value into memory.
        // Global value
        if kicked_value.is_global() {
            codes.extend(
                self.store_global(reg, value_data.name().clone().unwrap()[1..].to_string()),
            );
        }
        // Local or temp value
        else {
            let location = self.local_value_location.get(&kicked_value);
            if location.is_none() {
                // Already removed. No need to save it.
            } else {
                // Save it.
                codes.extend(self.store_with_offset(
                    reg,
                    *self.local_value_location.get(&kicked_value).unwrap() as isize,
                ));
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
        do_load: bool,
        use_certain_reg: Option<usize>,
    ) -> (usize, Vec<String>) {
        self.curr_time += 1;
        let value_data = match value.is_global() {
            true => self.program.borrow_value(value).clone(),
            false => self.fd.dfg().value(value).clone(),
        };
        if let koopa::ir::ValueKind::Integer(int) = value_data.kind() {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            let (dst_reg, mut codes) = match use_certain_reg {
                Some(reg) => (reg, vec![]),
                None => self.get_tmp_reg(),
            };
            codes.push(format!(
                "  li\t{}, {}",
                REGISTER_NAMES[dst_reg],
                int.value()
            ));
            self.__update_user(dst_reg, value);
            return (dst_reg, codes);
        }
        // Value already in a register
        if let Some(src_reg) = self.__is_value_in_register(value) {
            match use_certain_reg {
                Some(reg_dst) => {
                    if src_reg != reg_dst {
                        self.__update_user(reg_dst, value);
                        // self.__free_user(src_reg); Don't do that! May be used several times.
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
            match value.is_global() {
                true => {
                    codes.extend(
                        self.load_global(reg, value_data.name().clone().unwrap()[1..].to_string()),
                    );
                }
                false => {
                    let offset = self
                        .local_value_location
                        .get(&value)
                        .expect(
                            format!(
                                "Cannot find local or temp value {:?} in table! Impossible.",
                                value_data
                            )
                            .as_str(),
                        )
                        .clone();
                    codes.extend(self.load_with_offset(reg, offset as isize));
                }
            }
        }
        self.__update_user(reg, value);
        (reg, codes)
    }

    fn assign_v1_to_v2(&mut self, v1: Value, v2: Value) -> Vec<String> {
        let mut codes = vec![];
        let (reg1, codes1) = self.want_to_visit_value(v1, true, None);
        let (reg2, codes2) = self.want_to_visit_value(v2, false, None);
        codes.extend(codes1);
        codes.extend(codes2);
        codes.push(format!(
            "  mv\t{}, {}",
            REGISTER_NAMES[reg2], REGISTER_NAMES[reg1]
        ));
        codes
    }

    /// Example:
    /// getelemptr @a, 114 -> The absolute location of @a in memory.
    /// getelemptr %0, 514 -> The value of %0 (which is an address).
    fn get_absolute_location(&mut self, value: Value) -> (usize, Vec<String>) {
        let (reg, mut codes) = self.get_tmp_reg();
        if value.is_global() {
            codes.push(format!(
                "  la\t{}, {}",
                REGISTER_NAMES[reg],
                &self.program.borrow_value(value).name().clone().unwrap()[1..]
            ));
        } else {
            if self.is_temp_value(value) {
                let (_, codes_to_visit) = self.want_to_visit_value(value, true, Some(reg));
                codes.extend(codes_to_visit);
            } else {
                let offset = *self.local_value_location.get(&value).unwrap();
                if offset < MAX_SHORT_INT as usize {
                    codes.push(format!("  addi\t{}, sp, {}", REGISTER_NAMES[reg], offset));
                } else {
                    codes.push(format!(
                        "  li\t{}, {}\n  add\t{}, sp, {}",
                        REGISTER_NAMES[reg], offset, REGISTER_NAMES[reg], REGISTER_NAMES[reg]
                    ));
                }
            };
        }
        (reg, codes)
    }
}
