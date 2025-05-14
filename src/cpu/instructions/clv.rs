use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn clv_0xb8(cpu: &mut Cpu) -> u8 {
    cpu.flags.v = 0;
    trace_instruction!(cpu, "CLV", "0xB8", "Implied");
    2
}
