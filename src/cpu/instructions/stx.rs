use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_zeropage, addr_zeropage_y},
};
use crate::trace_instruction;

pub(crate) fn stx_0x86(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.x;

    trace_instruction!(cpu, "STX", "0x86", "Zeropage");
    3
}

pub(crate) fn stx_0x96(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_y(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.x;

    trace_instruction!(cpu, "STX", "0x96", "Zeropage,Y");
    4
}

pub(crate) fn stx_0x8e(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.x;

    trace_instruction!(cpu, "STX", "0x8E", "Absolute");
    4
}
