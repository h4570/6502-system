use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn clv_0xb8(cpu: &mut Cpu) -> u8 {
    cpu.flags.v = 0;
    trace!("CLC - Clear overflow flag");
    2
}
