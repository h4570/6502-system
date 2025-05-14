#[cfg(test)]
mod tsx_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_tsx_basic() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.sp = 0x42;
        let program = vec![0xBA, 0x00]; // TSX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.sp, 0x42);
        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tsx_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.sp = 0x00;
        let program = vec![0xBA, 0x00]; // TSX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.sp, 0x00);
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_tsx_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.sp = 0x80;
        let program = vec![0xBA, 0x00]; // TSX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.sp, 0x80);
        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
