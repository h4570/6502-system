use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn nop_0xea(_cpu: &mut Cpu) -> u8 {
    trace!("NOP[0xEA] Implied");
    2
}
