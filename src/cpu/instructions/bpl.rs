use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn bpl_0x10(cpu: &mut Cpu) -> u8 {
    // Branch if negative flag is clear (N=0)
    branch_if(cpu, cpu.flags.n == 0, "BPL")
}
