#[cfg(test)]
mod dec_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_dec_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x05;
        let program = vec![0xC6, 0x42, 0x00]; // DEC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x04);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dec_zeropage_zero_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x01;
        let program = vec![0xC6, 0x42, 0x00]; // DEC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_dec_zeropage_negative_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x00;
        let program = vec![0xC6, 0x42, 0x00]; // DEC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0xFF);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_dec_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x10; // 0x42 + 0x05 = 0x47
        let program = vec![0xD6, 0x42, 0x00]; // DEC $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x47], 0x0F);
    }

    #[test]
    fn test_dec_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x4242] = 0x42;
        let program = vec![0xCE, 0x42, 0x42, 0x00]; // DEC $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4242], 0x41);
    }

    #[test]
    fn test_dec_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x42; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xDE, 0x42, 0x42, 0x00]; // DEC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x424A], 0x41);
    }
}
