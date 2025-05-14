#[cfg(test)]
mod rol_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_rol_accumulator() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x45; // 01000101
        cpu.flags.c = 0;
        let program = vec![0x2A, 0x00]; // ROL A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x8A); // 10001010
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_rol_accumulator_with_carry() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x85; // 10000101
        cpu.flags.c = 1;
        let program = vec![0x2A, 0x00]; // ROL A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x0B); // 00001011
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_rol_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.a = 0x80; // 10000000
        cpu.flags.c = 0;
        let program = vec![0x2A, 0x00]; // ROL A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.z, 1);
    }

    #[test]
    fn test_rol_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.memory.data[0x42] = 0x45; // 01000101
        cpu.flags.c = 0;
        let program = vec![0x26, 0x42, 0x00]; // ROL $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x8A); // 10001010
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_rol_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x45; // 01000101, 0x42 + 0x05 = 0x47
        cpu.flags.c = 0;
        let program = vec![0x36, 0x42, 0x00]; // ROL $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x47], 0x8A); // 10001010
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_rol_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.memory.data[0x4242] = 0x45; // 01000101
        cpu.flags.c = 0;
        let program = vec![0x2E, 0x42, 0x42, 0x00]; // ROL $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4242], 0x8A); // 10001010
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }

    #[test]
    fn test_rol_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        // Setting initial values
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x45; // 01000101, 0x4242 + 0x08 = 0x424A
        cpu.flags.c = 0;
        let program = vec![0x3E, 0x42, 0x42, 0x00]; // ROL $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x424A], 0x8A); // 10001010
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.z, 0);
    }
}
