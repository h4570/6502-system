use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_absolute_x, addr_zeropage, addr_zeropage_x},
};

pub(crate) fn asl_0x0a(cpu: &mut Cpu) -> u8 {
    let val = cpu.registers.a;

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = (val >> 7) & 1;

    // Shift left one bit
    let result = val << 1;

    // Update accumulator
    cpu.registers.a = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    trace!("ASL[0x0A] Accumulator");
    2
}

pub(crate) fn asl_0x06(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = (val >> 7) & 1;

    // Shift left one bit
    let result = val << 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    trace!("ASL[0x06] Zeropage");
    5
}

pub(crate) fn asl_0x16(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = (val >> 7) & 1;

    // Shift left one bit
    let result = val << 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    trace!("ASL[0x16] Zeropage,X");
    6
}

pub(crate) fn asl_0x0e(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = (val >> 7) & 1;

    // Shift left one bit
    let result = val << 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    trace!("ASL[0x0E] Absolute");
    6
}

pub(crate) fn asl_0x1e(cpu: &mut Cpu) -> u8 {
    let (addr, _) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];

    // Set carry flag to the bit that will be shifted out
    cpu.flags.c = (val >> 7) & 1;

    // Shift left one bit
    let result = val << 1;

    // Update memory
    cpu.memory.data[addr as usize] = result;

    // Set zero and negative flags based on result
    cpu.flags.z = if result == 0 { 1 } else { 0 };
    cpu.flags.n = if (result & 0x80) != 0 { 1 } else { 0 };

    trace!("ASL[0x1E] Absolute,X");
    7
}
