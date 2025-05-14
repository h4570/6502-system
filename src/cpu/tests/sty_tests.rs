#[cfg(test)]
mod sty_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_sty_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x42;
        let program = vec![0x84, 0x20, 0x00]; // STY $20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x20], 0x42);
    }

    #[test]
    fn test_sty_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x42;
        cpu.registers.x = 0x05;
        let program = vec![0x94, 0x20, 0x00]; // STY $20,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x25], 0x42); // 0x20 + 0x05 = 0x25
    }

    #[test]
    fn test_sty_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x42;
        let program = vec![0x8C, 0x00, 0x40, 0x00]; // STY $4000, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4000], 0x42);
    }
}
