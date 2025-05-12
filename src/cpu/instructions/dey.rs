use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn dey_0x88(cpu: &mut Cpu) -> u8 {
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    
    cpu.flags.z = if cpu.registers.y == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.y & 0x80) != 0 { 1 } else { 0 };
    
    trace!("DEY[0x88] Implied");
    2
}
