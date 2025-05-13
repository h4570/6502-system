use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn clc_0x18(cpu: &mut Cpu) -> u8 {
    cpu.flags.c = 0;
    trace!("CLC - Clear carry flag");
    2
}
