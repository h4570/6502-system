use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn tya_0x98(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = cpu.registers.y;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };

    trace!("TYA[0x98] Implied");
    2
}
