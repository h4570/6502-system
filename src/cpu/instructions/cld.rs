use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn cld_0xd8(cpu: &mut Cpu) -> u8 {
    cpu.flags.d = 0;
    trace_instruction!(cpu, "CLD", "0xD8", "Implied");
    2
}
