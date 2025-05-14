use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn txs_0x9a(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.x;

    trace_instruction!(cpu, "TXS", "0x9A", "Implied");
    2
}
