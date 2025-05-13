use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn bne_0xd0(cpu: &mut Cpu) -> u8 {
    // Branch if zero flag is clear (Z=0)
    branch_if(cpu, cpu.flags.z == 0, "BNE")
}
