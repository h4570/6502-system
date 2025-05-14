#[cfg(test)]
mod ror_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_ror_accumulator() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x8A; // 10001010
        cpu.flags.c = 0;
        let program = vec![0x6A, 0x00]; // ROR A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x45); // 01000101
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_ror_accumulator_with_carry() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x45; // 01000101
        cpu.flags.c = 1;
        let program = vec![0x6A, 0x00]; // ROR A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0xA2); // 10100010
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_ror_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x01; // 00000001
        cpu.flags.c = 0;
        let program = vec![0x6A, 0x00]; // ROR A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 1);
    }

    #[test]
    fn test_ror_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.memory.data[0x42] = 0x8A; // 10001010
        cpu.flags.c = 0;
        let program = vec![0x66, 0x42, 0x00]; // ROR $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.memory.data[0x42], 0x45); // 01000101
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_ror_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x8A; // 10001010, 0x42 + 0x05 = 0x47
        cpu.flags.c = 0;
        let program = vec![0x76, 0x42, 0x00]; // ROR $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.memory.data[0x47], 0x45); // 01000101
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_ror_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.memory.data[0x4242] = 0x8A; // 10001010
        cpu.flags.c = 0;
        let program = vec![0x6E, 0x42, 0x42, 0x00]; // ROR $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.memory.data[0x4242], 0x45); // 01000101
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_ror_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x8A; // 10001010, 0x4242 + 0x08 = 0x424A
        cpu.flags.c = 0;
        let program = vec![0x7E, 0x42, 0x42, 0x00]; // ROR $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.memory.data[0x424A], 0x45); // 01000101
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }
}
