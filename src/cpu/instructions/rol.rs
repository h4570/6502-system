use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

fn rol(cpu: &mut Cpu, val: u8) -> u8 {
    let old_carry = cpu.flags.c;
    // Set carry flag to the value of bit 7
    cpu.flags.c = (val >> 7) & 1;
    // Shift left and add old carry to bit 0
    let result = (val << 1) | old_carry;

    // Set zero and negative flags
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    result
}

// ROL Accumulator
pub(crate) fn rol_0x2a(cpu: &mut Cpu) -> u8 {
    let val = cpu.registers.a;
    cpu.registers.a = rol(cpu, val);

    trace!("ROL[0x2A] Accumulator");
    2
}

// ROL Zero Page
pub(crate) fn rol_0x26(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = rol(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROL[0x26] Zero Page");
    5
}

// ROL Zero Page,X
pub(crate) fn rol_0x36(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = rol(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROL[0x36] Zero Page,X");
    6
}

// ROL Absolute
pub(crate) fn rol_0x2e(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = rol(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROL[0x2E] Absolute");
    6
}

// ROL Absolute,X
pub(crate) fn rol_0x3e(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = rol(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROL[0x3E] Absolute,X");
    7
}
