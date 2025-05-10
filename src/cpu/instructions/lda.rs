use crate::cpu::cpu::Cpu;

// TODO: pub(crate) daje wjazd z zewnątrz

impl Cpu {
    pub(crate) fn lda_0xa9(&mut self) -> u8 {
        let val: u8 = self.fetch_byte();
        self.registers.a = val;
        println!("Opcode: 0xA9");
        2
    }
}
