#[cfg(test)]
mod cli_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_cli() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.i = 1;
        let program = vec![0x58, 0x00]; // CLI, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.i, 0);
    }

    #[test]
    fn test_cli_already_clear() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.i = 0;
        let program = vec![0x58, 0x00]; // CLI, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.flags.i, 0);
    }
}
