use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn sed_0xf8(cpu: &mut Cpu) -> u8 {
    cpu.flags.d = 1;

    trace_instruction!(cpu, "SED", "0xF8", "Implied");
    2
}
