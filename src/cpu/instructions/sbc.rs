use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_val(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a;
    // Invert bits for subtraction and add 1 if carry is set
    let operand_comp = (!operand) as usize;
    let res_usize = (a as usize) + operand_comp + (cpu.flags.c as usize);
    let res = res_usize as u8; // This automatically handles the wrap-around

    cpu.registers.a = res;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };
    cpu.flags.c = if res_usize > 0xFF { 1 } else { 0 };

    // Set overflow flag (V)
    // V is set when the sign of the result differs from the sign of A
    // when the sign of the operand's complement differs from the sign of A
    let a_sign = (a & 0x80) != 0;
    let operand_comp_sign = (operand_comp as u8 & 0x80) != 0;
    let result_sign = (res & 0x80) != 0;

    // V set when A and operand_comp have same sign but result has different sign
    cpu.flags.v = if (a_sign == operand_comp_sign) && (a_sign != result_sign) {
        1
    } else {
        0
    };
}

pub(crate) fn sbc_0xe9(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);

    set_val(cpu, operand);

    trace!("SBC[0xE9] Immediate");
    2
}

pub(crate) fn sbc_0xe5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xE5] Zeropage");
    3
}

pub(crate) fn sbc_0xf5(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xF5] Zeropage,X");
    4
}

pub(crate) fn sbc_0xed(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xED] Absolute");
    4
}

pub(crate) fn sbc_0xfd(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xFD] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn sbc_0xf9(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xF9] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn sbc_0xe1(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xE1] (Indirect,X)");
    6
}

pub(crate) fn sbc_0xf1(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("SBC[0xF1] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
