use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn dex_0xca(cpu: &mut Cpu) -> u8 {
    cpu.registers.x = cpu.registers.x.wrapping_sub(1);

    cpu.flags.z = if cpu.registers.x == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.x & 0x80) != 0 { 1 } else { 0 };

    trace!("DEX[0xCA] Implied");
    2
}
