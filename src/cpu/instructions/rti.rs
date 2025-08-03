use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn rti_0x40(cpu: &mut Cpu) -> u8 {
    // Pull status register from stack
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let status = cpu.memory.data[0x0100 + cpu.registers.sp as usize];

    // [N][V][1][B][D][I][Z][C] (B flag ignored for RTI)
    cpu.flags.n = (status >> 7) & 1;
    cpu.flags.v = (status >> 6) & 1;
    cpu.flags.d = (status >> 3) & 1;
    cpu.flags.i = (status >> 2) & 1;
    cpu.flags.z = (status >> 1) & 1;
    cpu.flags.c = status & 1;

    // Pull program counter from stack - low byte first, then high byte
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let pc_lo = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let pc_hi = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;

    // RTI restores the exact return address from the stack without adjustment.
    // Unlike RTS, which increments the restored address by 1 (PC+1), RTI should not adjust the PC.
    cpu.registers.pc = (pc_hi << 8) | pc_lo;

    trace_instruction!(cpu, "RTI", "0x40", "Implied");
    6
}
