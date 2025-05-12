#[cfg(test)]
mod dex_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_dex_basic() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        let program = vec![0xCA, 0x00]; // DEX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x04);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dex_zero_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x01;
        let program = vec![0xCA, 0x00]; // DEX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dex_negative_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x00;
        let program = vec![0xCA, 0x00]; // DEX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0xFF);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_dex_wrap_around() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x00;
        let program = vec![0xCA, 0xCA, 0x00]; // DEX, DEX, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0xFE);
    }
}
