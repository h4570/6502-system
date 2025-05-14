use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn tya_0x98(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = cpu.registers.y;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };

    trace_instruction!(cpu, "TYA", "0x98", "Implied");
    2
}
