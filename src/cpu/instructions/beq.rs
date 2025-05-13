use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn beq_0xf0(cpu: &mut Cpu) -> u8 {
    // Branch if zero flag is set (Z=1)
    branch_if(cpu, cpu.flags.z == 1, "BEQ")
}
