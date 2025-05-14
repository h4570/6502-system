use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn pha_0x48(cpu: &mut Cpu) -> u8 {
    let val = cpu.registers.a;

    // Push accumulator value onto the stack
    cpu.memory.data[0x0100 + cpu.registers.sp as usize] = val;
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);

    trace!("PHA[0x48]");
    3
}
