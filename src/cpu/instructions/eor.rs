use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_val(cpu: &mut Cpu, operand: u8) {
    cpu.registers.a = cpu.registers.a ^ operand;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };
}

pub(crate) fn eor_0x49(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);

    set_val(cpu, operand);

    trace!("EOR[0x49] Immediate");
    2
}

pub(crate) fn eor_0x45(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x45] Zeropage");
    3
}

pub(crate) fn eor_0x55(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x55] Zeropage,X");
    4
}

pub(crate) fn eor_0x4d(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x4D] Absolute");
    4
}

pub(crate) fn eor_0x5d(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x5D] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn eor_0x59(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x59] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn eor_0x41(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x41] (Indirect,X)");
    6
}

pub(crate) fn eor_0x51(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("EOR[0x51] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
