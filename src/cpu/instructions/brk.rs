use crate::cpu::cpu::Cpu;

// TODO: pub(crate) daje wjazd z zewnątrz

impl Cpu {
    pub(crate) fn brk_0x00(&mut self) -> u8 {
        self.exit = true;
        println!("Opcode: 0x00");
        1
    }
}
