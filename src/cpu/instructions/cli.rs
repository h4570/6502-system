use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn cli_0x58(cpu: &mut Cpu) -> u8 {
    cpu.flags.i = 0;
    trace_instruction!(cpu, "CLI", "0x58", "Implied");
    2
}
