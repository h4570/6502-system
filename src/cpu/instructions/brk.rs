use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn brk_0x00(cpu: &mut Cpu) -> u8 {
    cpu.exit = true;
    trace_instruction!(cpu, "BRK", "0x00", "Implied");
    1
}
