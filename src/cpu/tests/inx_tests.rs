#[cfg(test)]
mod inx_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_inx_basic() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        let program = vec![0xE8, 0x00]; // INX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x06);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inx_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0xFF;
        let program = vec![0xE8, 0x00]; // INX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inx_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x7F;
        let program = vec![0xE8, 0x00]; // INX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
