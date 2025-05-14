use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn rts_0x60(cpu: &mut Cpu) -> u8 {
    // Pull return address from stack
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let lo = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;

    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let hi = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;

    // Construct the return address
    let return_addr = (hi << 8) | lo;

    // The 6502 CPU stores return address - 1 on the stack
    // So we need to add 1 to get back to the proper return point
    cpu.registers.pc = return_addr + 1;

    trace!("RTS[0x60] Returning to {:04X}", cpu.registers.pc);
    6
}
