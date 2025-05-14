use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn tax_0xaa(cpu: &mut Cpu) -> u8 {
    cpu.registers.x = cpu.registers.a;

    cpu.flags.z = if cpu.registers.x == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.x & 128) != 0 { 1 } else { 0 };

    trace_instruction!(cpu, "TAX", "0xAA", "Implied");
    2
}
