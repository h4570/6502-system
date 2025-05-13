use crate::cpu::cpu::Cpu;
use log::trace;

/// Common branch logic for all branch instructions.
///
/// Takes the CPU, the branch condition, and the opcode name for tracing.
/// Returns the number of cycles consumed by the branch operation.
pub fn branch_if(cpu: &mut Cpu, condition: bool, opcode_name: &str) -> u8 {
    let offset = cpu.fetch_byte() as i8;
    let pc = cpu.registers.pc;

    if condition {
        // Branch is taken
        let new_pc = ((pc as i32) + (offset as i32)) as u16;
        cpu.registers.pc = new_pc;

        // Page crossing check
        if (pc & 0xFF00) != (new_pc & 0xFF00) {
            trace!("{}[*] Relative (page crossed)", opcode_name);
            return 4; // Additional cycle for page crossing
        } else {
            trace!("{}[*] Relative", opcode_name);
            return 3; // Branch taken but no page crossing
        }
    }

    trace!("{}[*] Relative (branch not taken)", opcode_name);
    2 // Branch not taken
}
