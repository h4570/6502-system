use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn beq_0xf0(cpu: &mut Cpu) -> u8 {
    // Branch if zero flag is set (Z=1)
    let cycles = branch_if(cpu, cpu.flags.z == 1, "BEQ");
    trace_instruction!(cpu, "BEQ", "0xF0", "Relative");
    cycles
}
