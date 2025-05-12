use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn tax_0xaa(cpu: &mut Cpu) -> u8 {
    cpu.registers.x = cpu.registers.a;

    cpu.flags.z = if cpu.registers.x == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.x & 128) != 0 { 1 } else { 0 };

    trace!("TAX[0xAA] Implied");
    2
}
