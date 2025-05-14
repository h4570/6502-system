use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bcs_0xb0(cpu: &mut Cpu) -> u8 {
    // Branch if carry flag is set (C=1)
    let cycles = branch_if(cpu, cpu.flags.c == 1, "BCS");
    trace_instruction!(cpu, "BCS", "0xB0", "Relative");
    cycles
}
