use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn cld_0xd8(cpu: &mut Cpu) -> u8 {
    cpu.flags.d = 0;
    trace!("CLC - Clear decimal flag");
    2
}
