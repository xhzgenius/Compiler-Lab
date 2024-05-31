//! This module is the backend of my compiler.
//! It converts the Koopa IR into assembly code.

mod build_assembly;
use core::panic;
use std::collections::HashMap;
use std::io::Write;

use build_assembly::AssemblyBuildable;
use koopa::ir::{FunctionData, Program, TypeKind, Value};

pub fn generate_assembly(program: &Program, output_file: &mut std::fs::File) -> Result<(), String> {
    let codes = program.build()?;
    for code in codes {
        writeln!(output_file, "{}", code).expect("Write error. ");
    }
    Ok(())
}

const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

const REGISTER_FOR_TEMP: [usize; 15] = [5, 6, 7, 28, 29, 30, 31, 10, 11, 12, 13, 14, 15, 16, 17];

#[derive(Debug)]
pub struct FuncValueTable {
    curr_time: i32,
    register_user: [Option<Value>; 32],
    register_used_time: [i32; 32], // LRU registers
    value_location: HashMap<Value, usize>,
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
        for i in REGISTER_FOR_TEMP {
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

    /// Finds a usable register.
    /// If all registers are being used, then kicks one and returns the kicked register and its former user (Value).
    /// Else returns the empty register and None.
    /// This function should not be called outside.
    fn __get_usable_register(&mut self) -> (usize, Vec<String>) {
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
        // Kick a value and store it to the stack.
        let kicked_value = self.register_user[choice].unwrap();
        self.register_user[choice] = None;
        self.register_used_time[choice] = 0;
        let store_location = self.value_location.get(&kicked_value).unwrap();
        let mut codes = vec![];
        codes.push(format!(
            "  sw\t{}, {}(sp)",
            REGISTER_NAMES[choice], store_location
        ));
        (choice, codes)
    }

    /// Want to visit a value. Make it appear in a register.
    fn want_to_visit_value(
        &mut self,
        value: Value,
        fd: &FunctionData,
        do_load: bool,
    ) -> (usize, Vec<String>) {
        // Value is a constant
        if let koopa::ir::ValueKind::Integer(int) = fd.dfg().value(value).kind() {
            // Allocate a new register for the Integer.
            // I don't want to use assembly codes like addi because I am lazy.
            if int.value() == 0 {
                return (0, vec![]); // Register x0(id=0) is always 0.
            } else {
                // Allocate a new register for the constant integer.
                let (reg, mut codes) = self.get_tmp_reg();
                codes.push(format!("  li\t{}, {}", REGISTER_NAMES[reg], int.value()));
                return (reg, codes);
            }
        }
        // Value already in a register
        if let Some(reg) = self.__is_value_in_register(value) {
            return (reg, vec![]);
        }
        // Value not in registers
        let (reg, mut codes) = self.__get_usable_register();
        if do_load {
            let old_location = self
                .value_location
                .get(&value)
                .expect("Can't find value in table. ");
            codes.push(format!(
                "  lw\t{}, {}(sp)",
                REGISTER_NAMES[reg], old_location,
            ));
        }
        self.register_user[reg] = Some(value);
        self.register_used_time[reg] = self.curr_time;
        (reg, codes)
    }

    fn get_tmp_reg(&mut self) -> (usize, Vec<String>) {
        self.__get_usable_register()
    }

    // /// Frees a register.
    // fn free_register(&mut self, reg: usize) {
    //     self.register_user[reg] = None;
    //     self.register_used_time[reg] = 0;
    // }
}
