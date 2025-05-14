use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};
use crate::trace_instruction;

fn set_val(cpu: &mut Cpu, operand: u8) {
    cpu.registers.a = cpu.registers.a | operand;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };
}

pub(crate) fn ora_0x09(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x09", "Immediate");
    2
}

pub(crate) fn ora_0x05(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x05", "Zeropage");
    3
}

pub(crate) fn ora_0x15(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x15", "Zeropage,X");
    4
}

pub(crate) fn ora_0x0d(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x0D", "Absolute");
    4
}

pub(crate) fn ora_0x1d(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x1D", "Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn ora_0x19(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x19", "Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn ora_0x01(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x01", "(Indirect,X)");
    6
}

pub(crate) fn ora_0x11(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace_instruction!(cpu, "ORA", "0x11", "(Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
