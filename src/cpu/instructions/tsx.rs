use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn tsx_0xba(cpu: &mut Cpu) -> u8 {
    cpu.registers.x = cpu.registers.sp;

    cpu.flags.z = if cpu.registers.x == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.x & 128) != 0 { 1 } else { 0 };

    trace!("TSX[0xBA] Implied");
    2
}
