use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_immediate, addr_zeropage},
};
use crate::trace_instruction;

fn set_flags(cpu: &mut Cpu, operand: u8) {
    let x = cpu.registers.x;
    let result = x.wrapping_sub(operand);

    // Set zero flag if X equals operand
    cpu.flags.z = if x == operand { 1 } else { 0 };

    // Set negative flag if bit 7 of result is set
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    // Set carry flag if X >= operand (no borrow required)
    cpu.flags.c = if x >= operand { 1 } else { 0 };
}

pub(crate) fn cpx_0xe0(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPX", "0xE0", "Immediate");
    2
}

pub(crate) fn cpx_0xe4(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPX", "0xE4", "Zeropage");
    3
}

pub(crate) fn cpx_0xec(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, operand);

    trace_instruction!(cpu, "CPX", "0xEC", "Absolute");
    4
}
