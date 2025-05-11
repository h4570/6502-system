use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn brk_0x00(cpu: &mut Cpu) -> u8 {
    cpu.exit = true;
    trace!("BRK[0x00]");
    1
}
