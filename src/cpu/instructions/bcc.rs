use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn bcc_0x90(cpu: &mut Cpu) -> u8 {
    // Branch if carry flag is clear (C=0)
    branch_if(cpu, cpu.flags.c == 0, "BCC")
}
