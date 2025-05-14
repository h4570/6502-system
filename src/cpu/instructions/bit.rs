use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_zeropage},
};

pub(crate) fn bit_0x24(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];

    // AND with accumulator to set zero flag, but don't change A
    let result = cpu.registers.a & val;
    cpu.flags.z = if result == 0 { 1 } else { 0 };

    // Bits 7 and 6 of the value are copied to the N and V flags
    cpu.flags.n = (val >> 7) & 1;
    cpu.flags.v = (val >> 6) & 1;

    trace!("BIT[0x24] Zeropage");
    3
}

pub(crate) fn bit_0x2c(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];

    // AND with accumulator to set zero flag, but don't change A
    let result = cpu.registers.a & val;
    cpu.flags.z = if result == 0 { 1 } else { 0 };

    // Bits 7 and 6 of the value are copied to the N and V flags
    cpu.flags.n = (val >> 7) & 1;
    cpu.flags.v = (val >> 6) & 1;

    trace!("BIT[0x2C] Absolute");
    4
}
