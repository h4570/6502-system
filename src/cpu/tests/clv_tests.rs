#[cfg(test)]
mod clv_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_clv() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.v = 1;
        let program = vec![0xB8, 0x00]; // CLV, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.v, 0);
    }

    #[test]
    fn test_clv_already_clear() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.flags.v = 0;
        let program = vec![0xB8, 0x00]; // CLV, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.v, 0);
    }
}
