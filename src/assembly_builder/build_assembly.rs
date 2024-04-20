//! Convert a single Koopa IR component into assembly code.

use koopa::ir::{FunctionData, Program};

pub trait AssemblyBuildable {
    fn build(&self) -> Result<Vec<String>, String>;
}

impl AssemblyBuildable for Program {
    fn build(&self) -> Result<Vec<String>, String> {
        let mut assembly_codes = Vec::<String>::new();
        // Assembly code of global variables
        // TODO

        // Assembly code of functions
        assembly_codes.push("  .text".to_string());
        for &func in self.func_layout() {
            assembly_codes.extend(self.func(func).build()?);
        }
        Ok(assembly_codes)
    }
}

impl AssemblyBuildable for FunctionData {
    fn build(&self) -> Result<Vec<String>, String> {
        let mut assembly_codes = Vec::<String>::new();
        assembly_codes.push(format!("  .global {}", &self.name()[1..]).to_string());
        assembly_codes.push(format!("{}:", &self.name()[1..]).to_string());

        for (&block, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let value_data = self.dfg().value(value); // A value in Koopa IR is an instruction. 
                match value_data.kind() { // DO different things based on instruction kind. 
                    // Return instruction
                    koopa::ir::ValueKind::Return(return_inst) => {
                        match return_inst.value() { // Does it have a return value? 
                            Some(return_value) => {
                                let return_value_data = self.dfg().value(return_value);
                                match return_value_data.kind() { // Return value kind
                                    // Integer return value
                                    koopa::ir::ValueKind::Integer(int) => {
                                        assembly_codes.push(format!("  li a0, {}", int.value()));
                                    },
                                    // Other (TODO: not implemented)
                                    inst_kind => return Err(format!("Unknown Koopa IR instruction value {:?}", inst_kind))
                                }
                            },
                            None => {}
                        }
                        assembly_codes.push("  ret".to_string());
                    },
                    // Other instructions (TODO: Not implemented)
                    inst_kind => return Err(format!("Unknown Koopa IR instruction value {:?}", inst_kind))
                }
            }
        }
        Ok(assembly_codes)
    }
}
