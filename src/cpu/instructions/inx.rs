use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn inx_0xe8(cpu: &mut Cpu) -> u8 {
    cpu.registers.x = cpu.registers.x.wrapping_add(1);

    cpu.flags.z = if cpu.registers.x == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.x & 0x80) != 0 { 1 } else { 0 };

    trace!("INX[0xE8] Implied");
    2
}
