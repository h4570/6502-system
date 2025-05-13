use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;

pub(crate) fn bvc_0x50(cpu: &mut Cpu) -> u8 {
    // Branch if overflow flag is clear (V=0)
    branch_if(cpu, cpu.flags.v == 0, "BVC")
}
