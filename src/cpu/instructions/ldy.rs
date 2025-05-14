use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_immediate, addr_zeropage, addr_zeropage_x,
    },
};
use crate::trace_instruction;

fn set_val(cpu: &mut Cpu, val: u8) {
    cpu.registers.y = val;

    cpu.flags.z = if val == 0 { 1 } else { 0 };
    cpu.flags.n = if (val & 128) != 0 { 1 } else { 0 };
}

pub(crate) fn ldy_0xa0(cpu: &mut Cpu) -> u8 {
    let val = addr_immediate(cpu);
    set_val(cpu, val);

    trace_instruction!(cpu, "LDY", "0xA0", "Immediate");
    2
}

pub(crate) fn ldy_0xa4(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDY", "0xA4", "Zeropage");
    3
}

pub(crate) fn ldy_0xb4(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDY", "0xB4", "Zeropage,X");
    4
}

pub(crate) fn ldy_0xac(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDY", "0xAC", "Absolute");
    4
}

pub(crate) fn ldy_0xbc(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDY", "0xBC", "Absolute,X");
    if page_crossed { 5 } else { 4 }
}
