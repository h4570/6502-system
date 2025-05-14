use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn sed_0xf8(cpu: &mut Cpu) -> u8 {
    cpu.flags.d = 1;

    trace!("SED[0xF8] Implied");
    2
}
