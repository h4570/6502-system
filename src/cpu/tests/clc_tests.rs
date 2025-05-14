#[cfg(test)]
mod clc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_clc() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.c = 1;
        let program = vec![0x18, 0x00]; // CLC, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.c, 0);
    }

    #[test]
    fn test_clc_already_clear() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.c = 0;
        let program = vec![0x18, 0x00]; // CLC, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.c, 0);
    }
}
