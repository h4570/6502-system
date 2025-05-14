use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn rts_0x60(cpu: &mut Cpu) -> u8 {
    // Pull return address from stack
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let lo = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;
    
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let hi = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;
    
    // Return address is stored as PC - 1, so add 1
    cpu.registers.pc = ((hi << 8) | lo) + 1;
    
    trace!("RTS[0x60]");
    6
}
