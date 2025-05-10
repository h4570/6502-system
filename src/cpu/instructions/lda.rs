use crate::cpu::cpu::Cpu;

pub(crate) fn lda_0xa9(cpu: &mut Cpu) -> u8 {
    let val: u8 = cpu.fetch_byte();
    cpu.registers.a = val;
    println!("Opcode: 0xA9");
    2
}
