use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn sei_0x78(cpu: &mut Cpu) -> u8 {
    cpu.flags.i = 1;

    trace_instruction!(cpu, "SEI", "0x78", "Implied");
    2
}
