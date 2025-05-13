use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_val(cpu: &mut Cpu, val: u8) {
    cpu.registers.a = val;

    cpu.flags.z = if val == 0 { 1 } else { 0 };
    cpu.flags.n = if (val & 0x80) != 0 { 1 } else { 0 };
}

pub(crate) fn lda_0xa9(cpu: &mut Cpu) -> u8 {
    let val = addr_immediate(cpu);
    set_val(cpu, val);

    trace!("LDA[0xA9] Immediate");
    2
}

pub(crate) fn lda_0xa5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xA5] Zeropage");
    3
}

pub(crate) fn lda_0xb5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xB5] Zeropage,X");
    4
}

pub(crate) fn lda_0xad(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xAD] Absolute");
    4
}

pub(crate) fn lda_0xbd(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xBD] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn lda_0xb9(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xB9] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn lda_0xa1(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xA1] (Indirect,X)");
    6
}

pub(crate) fn lda_0xb1(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace!("LDA[0xB1] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
