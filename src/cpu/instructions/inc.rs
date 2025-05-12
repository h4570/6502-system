use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

fn increment_memory(cpu: &mut Cpu, addr: u16) {
    let value = cpu.memory.data[addr as usize];
    let result = value.wrapping_add(1);
    cpu.memory.data[addr as usize] = result;

    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };
}

pub(crate) fn inc_0xe6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    increment_memory(cpu, addr);

    trace!("INC[0xE6] Zeropage");
    5
}

pub(crate) fn inc_0xf6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    increment_memory(cpu, addr);

    trace!("INC[0xF6] Zeropage,X");
    6
}

pub(crate) fn inc_0xee(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    increment_memory(cpu, addr);

    trace!("INC[0xEE] Absolute");
    6
}

pub(crate) fn inc_0xfe(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    increment_memory(cpu, addr);

    trace!("INC[0xFE] Absolute,X");
    7
}
