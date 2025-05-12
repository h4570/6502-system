use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

fn set_flags(cpu: &mut Cpu, result: u8) {
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };
}

pub(crate) fn dec_0xc6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = val.wrapping_sub(1);
    cpu.memory.data[addr as usize] = result;

    set_flags(cpu, result);

    trace!("DEC[0xC6] Zeropage");
    5
}

pub(crate) fn dec_0xd6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = val.wrapping_sub(1);
    cpu.memory.data[addr as usize] = result;

    set_flags(cpu, result);

    trace!("DEC[0xD6] Zeropage,X");
    6
}

pub(crate) fn dec_0xce(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = val.wrapping_sub(1);
    cpu.memory.data[addr as usize] = result;

    set_flags(cpu, result);

    trace!("DEC[0xCE] Absolute");
    6
}

pub(crate) fn dec_0xde(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = val.wrapping_sub(1);
    cpu.memory.data[addr as usize] = result;

    set_flags(cpu, result);

    trace!("DEC[0xDE] Absolute,X");
    7
}
