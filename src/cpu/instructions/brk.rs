use crate::cpu::cpu::Cpu;

pub(crate) fn brk_0x00(cpu: &mut Cpu) -> u8 {
    cpu.exit = true;
    println!("Opcode: 0x00");
    1
}
