#[cfg(test)]
mod tya_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_tya_basic() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x42, 0x98, 0x00]; // LDY #$42, TYA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tya_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x00, 0x98, 0x00]; // LDY #$00, TYA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tya_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x80, 0x98, 0x00]; // LDY #$80, TYA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.y, 0x80);
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
