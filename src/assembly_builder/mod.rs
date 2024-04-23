//! This module is the backend of my compiler.
//! It converts the Koopa IR into assembly code.

mod build_assembly;
use build_assembly::AssemblyBuildable;
use koopa::ir::{Program, Value};

pub fn generate_assembly(program: &Program, output_file: std::fs::File) -> Result<(), String> {
    let mut my_agi = MyAssemblyGeneratorInfo {
        curr_time: 0,
        register_user: [None; 32],
        register_used_time: [0; 32],
        output_file: output_file,
    };
    program.build(&mut my_agi)?;
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
    output_file: std::fs::File,
}

impl MyAssemblyGeneratorInfo {
    /// Finds a usable register.
    /// If all registers are being used, then kicks one and returns the kicked register and its former user (Value).
    /// Else returns the empty register and None.
    /// This function should not be called outside.
    fn get_usable_register(&mut self) -> usize {
        self.curr_time += 1;
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
                None => return i,
            }
        }
        let _choice = choice.unwrap();
        // Kick a value and store it to the stack. TODO.
        todo!()
    }

    /// Finds where this value is stored, either in the registers or in the stack.
    fn find_using_register(&self, value: Value) -> Option<usize> {
        for i in 0..32 {
            if let Some(v) = self.register_user[i] {
                if v == value {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Allocates a register for a value.
    /// The register is automatically selected.
    fn allocate_register(&mut self, value: Value) -> usize {
        let reg = self.get_usable_register();
        self.register_user[reg] = Some(value);
        self.register_used_time[reg] = 0;
        reg
    }

    /// Frees a register. 
    fn free_register(&mut self, reg: usize) {
        self.register_user[reg] = None;
        self.register_used_time[reg] = 0;
    }
}
