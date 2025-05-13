use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn bcs_0xb0(cpu: &mut Cpu) -> u8 {
    // Branch if carry flag is set (C=1)
    branch_if(cpu, cpu.flags.c == 1, "BCS")
}
