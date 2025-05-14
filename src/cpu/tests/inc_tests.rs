#[cfg(test)]
mod inc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_inc_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x05;
        let program = vec![0xE6, 0x42, 0x00]; // INC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x06);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inc_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x05; // $42 + $05 = $47
        let program = vec![0xF6, 0x42, 0x00]; // INC $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x47], 0x06);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inc_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x4242] = 0x05;
        let program = vec![0xEE, 0x42, 0x42, 0x00]; // INC $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4242], 0x06);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x05; // $4242 + $08 = $424A
        let program = vec![0xFE, 0x42, 0x42, 0x00]; // INC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x424A], 0x06);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inc_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0xFF;
        let program = vec![0xE6, 0x42, 0x00]; // INC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_inc_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x7F;
        let program = vec![0xE6, 0x42, 0x00]; // INC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }
}
