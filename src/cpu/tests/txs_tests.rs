#[cfg(test)]
mod txs_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_txs_basic() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0x42, 0x9A, 0x00]; // LDX #$42, TXS, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.sp, 0x42);
    }

    #[test]
    fn test_txs_zero_value() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0x00, 0x9A, 0x00]; // LDX #$00, TXS, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.registers.sp, 0x00);
        // Note: TXS doesn't affect any flags
    }

    #[test]
    fn test_txs_high_value() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0xFF, 0x9A, 0x00]; // LDX #$FF, TXS, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0xFF);
        assert_eq!(cpu.registers.sp, 0xFF);
        // Note: TXS doesn't affect any flags
    }
}
