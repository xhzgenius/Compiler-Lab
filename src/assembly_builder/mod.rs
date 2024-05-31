//! This module is the backend of my compiler.
//! It converts the Koopa IR into assembly code.

mod build_assembly;
use std::collections::HashMap;
use std::io::Write;

use build_assembly::AssemblyBuildable;
use koopa::ir::{Program, Value};

pub fn generate_assembly(program: &Program, output_file: &mut std::fs::File) -> Result<(), String> {
    let codes = program.build()?;
    for code in codes {
        writeln!(output_file, "{}", code).expect("Write error. ");
    }
    Ok(())
}

const REGISTER_NAMES: [&str; 32] = [
    "x0", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5",
    "t6",
];

const REGISTER_FOR_TEMP: [usize; 15] = [5, 6, 7, 28, 29, 30, 31, 10, 11, 12, 13, 14, 15, 16, 17];

#[derive(Debug)]
pub struct MyAssemblyGeneratorInfo {
    curr_time: i32,
    register_user: [Option<Value>; 32],
    register_used_time: [i32; 32], // LRU registers
    local_var_location_in_stack: HashMap<String, usize>,
}

impl MyAssemblyGeneratorInfo {
    fn new() -> MyAssemblyGeneratorInfo {
        MyAssemblyGeneratorInfo {
            curr_time: 0,
            register_user: [None; 32],
            register_used_time: [0; 32],
            local_var_location_in_stack: HashMap::new(),
        }
    }
    /// Finds a usable register.
    /// If all registers are being used, then kicks one and returns the kicked register and its former user (Value).
    /// Else returns the empty register and None.
    /// This function should not be called outside.
    fn __get_usable_register(&mut self) -> Result<usize, String> {
        let mut now_min = std::i32::MAX;
        let mut choice: Option<usize> = None;
        for i in REGISTER_FOR_TEMP {
            match self.register_user[i] {
                Some(_) => {
                    if self.register_used_time[i] < now_min {
                        now_min = self.register_used_time[i];
                        choice = Some(i);
                    }
                }
                None => return Ok(i),
            }
        }
        let _choice = choice.unwrap();
        // Kick a value and store it to the stack. TODO.
        Err(format!(
            "Not enough register! This probably should not happen. "
        ))
    }

    /// Finds where this value is stored.
    /// If in stack, kick a value in a register and bring it back to the register.
    fn find_using_register(&self, value: Value) -> Result<usize, String> {
        for i in 0..32 {
            if let Some(v) = self.register_user[i] {
                if v == value {
                    return Ok(i);
                }
            }
        }
        // Value stored in the stack. Bring it back to a register.
        Err(format!(
            "Value not in register! This probably should not happen. "
        ))
    }

    /// Allocates a register for a value.
    /// The register is automatically selected.
    fn allocate_register(&mut self, value: Value) -> Result<usize, String> {
        let reg = self.__get_usable_register()?;
        self.register_user[reg] = Some(value);
        self.register_used_time[reg] = self.curr_time;
        Ok(reg)
    }

    /// Frees a register.
    fn free_register(&mut self, reg: usize) {
        self.register_user[reg] = None;
        self.register_used_time[reg] = 0;
    }
}
