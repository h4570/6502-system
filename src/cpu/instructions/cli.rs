use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn cli_0x58(cpu: &mut Cpu) -> u8 {
    cpu.flags.i = 0;
    trace!("CLC - Clear interrupt flag");
    2
}
