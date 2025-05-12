use log::trace;

use crate::cpu::cpu::Cpu;

pub(crate) fn txs_0x9a(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.x;

    trace!("TXS[0x9A] Implied");
    2
}
