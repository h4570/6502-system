use crate::cpu::cpu::Cpu;
use crate::trace_instruction;

pub(crate) fn jmp_0x4c(cpu: &mut Cpu) -> u8 {
    // Absolute addressing mode - 3 bytes (opcode + low byte + high byte)
    let lo = cpu.fetch_byte() as u16;
    let hi = cpu.fetch_byte() as u16;

    let addr = (hi << 8) | lo;
    cpu.registers.pc = addr;

    trace_instruction!(cpu, "JMP", "0x4C", "Absolute");
    3
}

pub(crate) fn jmp_0x6c(cpu: &mut Cpu) -> u8 {
    // Indirect addressing mode - 3 bytes (opcode + pointer low + pointer high)
    let lo = cpu.fetch_byte() as u16;
    let hi = cpu.fetch_byte() as u16;

    let pointer = (hi << 8) | lo;

    // Reproduce the 6502 indirect jump bug: if the pointer is at the end of a page,
    // it wraps around to the beginning of the same page rather than carrying to the next page
    let addr_lo = cpu.memory.data[pointer as usize] as u16;
    let addr_hi = if (pointer & 0xFF) == 0xFF {
        // The hardware bug: when pointer is $xxFF, fetch high byte from $xx00 instead of $(xx+1)00
        cpu.memory.data[(pointer & 0xFF00) as usize] as u16
    } else {
        cpu.memory.data[(pointer + 1) as usize] as u16
    };

    let addr = (addr_hi << 8) | addr_lo;
    cpu.registers.pc = addr;

    trace_instruction!(cpu, "JMP", "0x6C", "Indirect");
    5
}
