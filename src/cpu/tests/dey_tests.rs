#[cfg(test)]
mod dey_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_dey_basic() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x05;
        let program = vec![0x88, 0x00]; // DEY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x04);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dey_zero_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x01;
        let program = vec![0x88, 0x00]; // DEY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dey_negative_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x00;
        let program = vec![0x88, 0x00]; // DEY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0xFF);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_dey_wrap_around() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.y = 0x00;
        let program = vec![0x88, 0x88, 0x00]; // DEY, DEY, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0xFE);
    }
}
