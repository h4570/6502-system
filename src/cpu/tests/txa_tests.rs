#[cfg(test)]
mod txa_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_txa_basic() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0x42, 0x8A, 0x00]; // LDX #$42, TXA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_txa_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0x00, 0x8A, 0x00]; // LDX #$00, TXA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_txa_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA2, 0x80, 0x8A, 0x00]; // LDX #$80, TXA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
