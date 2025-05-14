use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn nop_0xea(cpu: &mut Cpu) -> u8 {
    trace_instruction!(cpu, "NOP", "0xEA", "Implied");
    2
}
