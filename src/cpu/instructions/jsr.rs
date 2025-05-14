use log::trace;

use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::addr_utils::addr_absolute;

pub(crate) fn jsr_0x20(cpu: &mut Cpu) -> u8 {
    let target_addr = addr_absolute(cpu);
    
    // The return address points to the last byte of the JSR instruction
    let return_addr = cpu.registers.pc - 1;
    
    // Push return address high byte first, then low byte
    cpu.memory.data[0x0100 + cpu.registers.sp as usize] = ((return_addr >> 8) & 0xFF) as u8;
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.memory.data[0x0100 + cpu.registers.sp as usize] = (return_addr & 0xFF) as u8;
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    
    // Jump to the target address
    cpu.registers.pc = target_addr;
    
    trace!("JSR[0x20] Absolute");
    6
}
