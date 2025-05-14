use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bcc_0x90(cpu: &mut Cpu) -> u8 {
    // Branch if carry flag is clear (C=0)
    let cycles = branch_if(cpu, cpu.flags.c == 0, "BCC");
    trace_instruction!(cpu, "BCC", "0x90", "Relative");
    cycles
}
