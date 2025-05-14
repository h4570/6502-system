use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn pla_0x68(cpu: &mut Cpu) -> u8 {
    // Increment stack pointer first, then pull value
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let val = cpu.memory.data[0x0100 + cpu.registers.sp as usize];

    cpu.registers.a = val;

    // Update flags based on the pulled value
    cpu.flags.z = if val == 0 { 1 } else { 0 };
    cpu.flags.n = if (val & 0x80) != 0 { 1 } else { 0 };

    trace!("PLA[0x68]");
    4
}
