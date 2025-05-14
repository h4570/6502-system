use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

pub(crate) fn lsr_0x4a(cpu: &mut Cpu) -> u8 {
    let val = cpu.registers.a;

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = val & 1;

    // Shift right one bit
    let result = val >> 1;

    // Update accumulator
    cpu.registers.a = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = 0; // Always clear for LSR

    trace!("LSR[0x4A] Accumulator");
    2
}

pub(crate) fn lsr_0x46(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = val & 1;

    // Shift right one bit
    let result = val >> 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = 0; // Always clear for LSR

    trace!("LSR[0x46] Zeropage");
    5
}

pub(crate) fn lsr_0x56(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = val & 1;

    // Shift right one bit
    let result = val >> 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = 0; // Always clear for LSR

    trace!("LSR[0x56] Zeropage,X");
    6
}

pub(crate) fn lsr_0x4e(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = val & 1;

    // Shift right one bit
    let result = val >> 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = 0; // Always clear for LSR

    trace!("LSR[0x4E] Absolute");
    6
}

pub(crate) fn lsr_0x5e(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = val & 1;

    // Shift right one bit
    let result = val >> 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = 0; // Always clear for LSR

    trace!("LSR[0x5E] Absolute,X");
    7
}
