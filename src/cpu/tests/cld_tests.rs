#[cfg(test)]
mod cld_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_cld() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.d = 1;
        let program = vec![0xD8, 0x00]; // CLD, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.d, 0);
    }

    #[test]
    fn test_cld_already_clear() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.d = 0;
        let program = vec![0xD8, 0x00]; // CLD, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.d, 0);
    }
}
