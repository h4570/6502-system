#[cfg(test)]
mod tax_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_tax_basic() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x42, 0xAA, 0x00]; // LDA #$42, TAX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tax_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x00, 0xAA, 0x00]; // LDA #$00, TAX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tax_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA9, 0x80, 0xAA, 0x00]; // LDA #$80, TAX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
