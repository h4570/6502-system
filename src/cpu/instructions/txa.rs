use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn txa_0x8a(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = cpu.registers.x;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };

    trace_instruction!(cpu, "TXA", "0x8A", "Implied");
    2
}
