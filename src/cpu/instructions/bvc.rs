use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bvc_0x50(cpu: &mut Cpu) -> u8 {
    // Branch if overflow flag is clear (V=0)
    let cycles = branch_if(cpu, cpu.flags.v == 0, "BVC");
    trace_instruction!(cpu, "BVC", "0x50", "Relative");
    cycles
}
