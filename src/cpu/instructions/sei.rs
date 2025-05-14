use log::trace;
use crate::cpu::cpu::Cpu;

pub(crate) fn sei_0x78(cpu: &mut Cpu) -> u8 {
    cpu.flags.i = 1;
    
    trace!("SEI[0x78] Implied");
    2
}
