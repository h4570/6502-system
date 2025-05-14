use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn sec_0x38(cpu: &mut Cpu) -> u8 {
    cpu.flags.c = 1;

    trace_instruction!(cpu, "SEC", "0x38", "Implied");
    2
}
