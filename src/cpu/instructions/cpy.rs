use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_immediate, addr_zeropage},
};
use crate::trace_instruction;

fn set_flags(cpu: &mut Cpu, operand: u8) {
    let y = cpu.registers.y;
    let result = y.wrapping_sub(operand);

    // Set zero flag if Y equals operand
    cpu.flags.z = if y == operand { 1 } else { 0 };

    // Set negative flag if bit 7 of result is set
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    // Set carry flag if Y >= operand (no borrow required)
    cpu.flags.c = if y >= operand { 1 } else { 0 };
}

pub(crate) fn cpy_0xc0(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPY", "0xC0", "Immediate");
    2
}

pub(crate) fn cpy_0xc4(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPY", "0xC4", "Zeropage");
    3
}

pub(crate) fn cpy_0xcc(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPY", "0xCC", "Absolute");
    4
}
