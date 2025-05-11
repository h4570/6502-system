use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_indirect_x, addr_indirect_y,
        addr_zeropage, addr_zeropage_x,
    },
};

pub(crate) fn sta_0x85(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x85] Zeropage");
    3
}

pub(crate) fn sta_0x95(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x95] Zeropage,X");
    4
}

pub(crate) fn sta_0x8d(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x8D] Absolute");
    4
}

pub(crate) fn sta_0x9d(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x9D] Absolute,X");
    5
}

pub(crate) fn sta_0x99(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_y(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x99] Absolute,Y");
    5
}

pub(crate) fn sta_0x81(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x81] (Indirect,X)");
    6
}

pub(crate) fn sta_0x91(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_indirect_y(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.a;

    trace!("STA[0x91] (Indirect),Y");
    6
}
