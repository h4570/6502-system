use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn sec_0x38(cpu: &mut Cpu) -> u8 {
    cpu.flags.c = 1;

    trace!("SEC[0x38] Implied");
    2
}
