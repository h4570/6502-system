use crate::trace_instruction;
use log::trace;

use crate::cpu::{
    cpu::Cpu,
    instructions::addr_utils::{
        addr_absolute, addr_absolute_x, addr_absolute_y, addr_immediate, addr_indirect_x,
        addr_indirect_y, addr_zeropage, addr_zeropage_x,
    },
};

fn set_val(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a;
    let res_usize = (a as usize) + (operand as usize) + (cpu.flags.c as usize);
    let res = res_usize as u8; // This automatically handles the wrap-around

    cpu.registers.a = res;

    cpu.flags.z = if cpu.registers.a == 0 { 1 } else { 0 };
    cpu.flags.n = if (cpu.registers.a & 128) != 0 { 1 } else { 0 };
    cpu.flags.c = if res_usize > 0xFF { 1 } else { 0 };

    // Set overflow flag (V)
    // V is set when the sign of the result differs from both the sign of A and operand
    // or when both operands have the same sign but the result has a different sign
    let a_sign = (a & 0x80) != 0;
    let operand_sign = (operand & 0x80) != 0;
    let result_sign = (res & 0x80) != 0;

    // V set when operands have same sign but result has different sign
    cpu.flags.v = if (a_sign == operand_sign) && (a_sign != result_sign) {
        1
    } else {
        0
    };
}

pub(crate) fn adc_0x69(cpu: &mut Cpu) -> u8 {
    let operand = addr_immediate(cpu);

    set_val(cpu, operand);

    trace_instruction!(cpu, ADC, "0x69 Immediate");
    2
}

pub(crate) fn adc_0x65(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x65] Zeropage");
    3
}

pub(crate) fn adc_0x75(cpu: &mut Cpu) -> u8 {
    let addr = addr_zeropage_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x75] Zeropage,X");
    4
}

pub(crate) fn adc_0x6d(cpu: &mut Cpu) -> u8 {
    let addr = addr_absolute(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x6D] Absolute");
    4
}

pub(crate) fn adc_0x7d(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x7D] Absolute,X");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn adc_0x79(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_absolute_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x79] Absolute,Y");
    if page_crossed { 5 } else { 4 }
}

pub(crate) fn adc_0x61(cpu: &mut Cpu) -> u8 {
    let addr = addr_indirect_x(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x61] (Indirect,X)");
    6
}

pub(crate) fn adc_0x71(cpu: &mut Cpu) -> u8 {
    let (addr, page_crossed) = addr_indirect_y(cpu);
    let operand = cpu.memory.data[addr as usize];

    set_val(cpu, operand);

    trace!("ADC[0x71] (Indirect),Y");
    if page_crossed { 6 } else { 5 }
}
