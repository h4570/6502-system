use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn tay_0xa8(cpu: &mut Cpu) -> u8 {
    cpu.registers.y = cpu.registers.a;

    cpu.flags.z = if cpu.registers.y == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.y & 128) != 0 { 1 } else { 0 };

    trace_instruction!(cpu, "TAY", "0xA8", "Implied");
    2
}
