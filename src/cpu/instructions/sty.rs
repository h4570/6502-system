use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{addr_absolute, addr_zeropage, addr_zeropage_x},
};
use crate::trace_instruction;

pub(crate) fn sty_0x84(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.y;

    trace_instruction!(cpu, "STY", "0x84", "Zeropage");
    3
}

pub(crate) fn sty_0x94(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.y;

    trace_instruction!(cpu, "STY", "0x94", "Zeropage,X");
    4
}

pub(crate) fn sty_0x8c(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    cpu.memory.data[addr as usize] = cpu.registers.y;

    trace_instruction!(cpu, "STY", "0x8C", "Absolute");
    4
}
