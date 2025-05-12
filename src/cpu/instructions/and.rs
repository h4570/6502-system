use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_val(cpu: &mut Cpu, operand: u8) {
    cpu.registers.a = cpu.registers.a & operand;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };
}

pub(crate) fn and_0x29(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);

    set_val(cpu, operand);

    trace!("AND[0x29] Immediate");
    2
}

pub(crate) fn and_0x25(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x25] Zeropage");
    3
}

pub(crate) fn and_0x35(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x35] Zeropage,X");
    4
}

pub(crate) fn and_0x2d(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x2D] Absolute");
    4
}

pub(crate) fn and_0x3d(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x3D] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn and_0x39(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x39] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn and_0x21(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x21] (Indirect,X)");
    6
}

pub(crate) fn and_0x31(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("AND[0x31] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
