#[cfg(test)]
mod tay_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_tay_basic() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x42, 0xA8, 0x00]; // LDA #$42, TAY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tay_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x00, 0xA8, 0x00]; // LDA #$00, TAY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tay_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x80, 0xA8, 0x00]; // LDA #$80, TAY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.registers.y, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
