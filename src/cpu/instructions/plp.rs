use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn plp_0x28(cpu: &mut Cpu) -> u8 {
    trace_instruction!(cpu, "PLP", "0x28", "Implied");

    // Pull status register from stack
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let status = cpu.memory.data[0x0100 + cpu.registers.sp as usize];

    // Update flags (not updating bits 4 and 5 as they are ignored)
    cpu.flags.n = (status >> 7) & 1;
    cpu.flags.v = (status >> 6) & 1;
    // Skip bit 5 (B flag) as it's not a real flag in the processor
    // Skip bit 4 (unused)
    cpu.flags.d = (status >> 3) & 1;
    cpu.flags.i = (status >> 2) & 1;
    cpu.flags.z = (status >> 1) & 1;
    cpu.flags.c = status & 1;

    4 // Cycles
}
