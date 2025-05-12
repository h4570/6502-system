use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_flags(cpu: &mut Cpu, register_val: u8, operand: u8) {
    let result = register_val.wrapping_sub(operand);

    // Set zero flag if register equals operand
    cpu.flags.z = if register_val == operand { 1 } else { 0 };

    // Set negative flag if bit 7 of result is set
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    // Set carry flag if register >= operand (no borrow required)
    cpu.flags.c = if register_val >= operand { 1 } else { 0 };
}

pub(crate) fn cmp_0xc9(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xC9] Immediate");
    2
}

pub(crate) fn cmp_0xc5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xC5] Zeropage");
    3
}

pub(crate) fn cmp_0xd5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xD5] Zeropage,X");
    4
}

pub(crate) fn cmp_0xcd(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xCD] Absolute");
    4
}

pub(crate) fn cmp_0xdd(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xDD] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn cmp_0xd9(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xD9] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn cmp_0xc1(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xC1] (Indirect,X)");
    6
}

pub(crate) fn cmp_0xd1(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];
    set_flags(cpu, cpu.registers.a, operand);

    trace!("CMP[0xD1] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
