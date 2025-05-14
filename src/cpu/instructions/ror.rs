use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

fn ror(cpu: &mut Cpu, val: u8) -> u8 {
    let old_carry = cpu.flags.c;
    // Set carry flag to the value of bit 0
    cpu.flags.c = val & 1;
    // Shift right and add old carry to bit 7
    let result = (val >> 1) | (old_carry << 7);

    // Set zero and negative flags
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    result
}

// ROR Accumulator
pub(crate) fn ror_0x6a(cpu: &mut Cpu) -> u8 {
    let val = cpu.registers.a;
    cpu.registers.a = ror(cpu, val);

    trace!("ROR[0x6A] Accumulator");
    2
}

// ROR Zero Page
pub(crate) fn ror_0x66(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = ror(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROR[0x66] Zero Page");
    5
}

// ROR Zero Page,X
pub(crate) fn ror_0x76(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = ror(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROR[0x76] Zero Page,X");
    6
}

// ROR Absolute
pub(crate) fn ror_0x6e(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = ror(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROR[0x6E] Absolute");
    6
}

// ROR Absolute,X
pub(crate) fn ror_0x7e(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];
    let result = ror(cpu, val);
    cpu.memory.data[addr as usize] = result;

    trace!("ROR[0x7E] Absolute,X");
    7
}
