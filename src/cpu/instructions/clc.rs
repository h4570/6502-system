use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn clc_0x18(cpu: &mut Cpu) -> u8 {
    cpu.flags.c = 0;
    trace_instruction!(cpu, "CLC", "0x18", "Implied");
    2
}
