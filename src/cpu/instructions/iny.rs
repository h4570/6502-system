use log::trace;
use crate::cpu::cpu::Cpu;

pub(crate) fn iny_0xc8(cpu: &mut Cpu) -> u8 {
    cpu.registers.y = cpu.registers.y.wrapping_add(1);

    cpu.flags.z = if cpu.registers.y == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.y & 0x80) != 0 { 1 } else { 0 };

    trace!("INY[0xC8] Implied");
    2
}
