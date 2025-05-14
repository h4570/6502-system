use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_y, addr_immediate, addr_zeropage, addr_zeropage_y,
    },
};
use crate::trace_instruction;

fn set_val(cpu: &mut Cpu, val: u8) {
    cpu.registers.x = val;

    cpu.flags.z = if val == 0 { 1 } else { 0 };
    cpu.flags.n = if (val & 128) != 0 { 1 } else { 0 };
}

pub(crate) fn ldx_0xa2(cpu: &mut Cpu) -> u8 {
    let val = addr_immediate(cpu);
    set_val(cpu, val);

    trace_instruction!(cpu, "LDX", "0xA2", "Immediate");
    2
}

pub(crate) fn ldx_0xa6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDX", "0xA6", "Zeropage");
    3
}

pub(crate) fn ldx_0xb6(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_y(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDX", "0xB6", "Zeropage,Y");
    4
}

pub(crate) fn ldx_0xae(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDX", "0xAE", "Absolute");
    4
}

pub(crate) fn ldx_0xbe(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let val = cpu.memory.data[addr as usize];
    set_val(cpu, val);

    trace_instruction!(cpu, "LDX", "0xBE", "Absolute,Y");
    if page_crossed { 5 } else { 4 }
}
