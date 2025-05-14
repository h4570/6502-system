#[cfg(test)]
mod asl_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40; // 01000000
        let program = vec![0x0A, 0x00]; // ASL A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x80); // 10000000
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }

    #[test]
    fn test_asl_accumulator_carry() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x80; // 10000000
        let program = vec![0x0A, 0x00]; // ASL A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x00); // 00000000
        assert_eq!(cpu.flags.c, 1); // Carry set
        assert_eq!(cpu.flags.z, 1); // Result is zero
        assert_eq!(cpu.flags.n, 0); // Negative flag clear
    }

    #[test]
    fn test_asl_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x40; // 01000000
        let program = vec![0x06, 0x42, 0x00]; // ASL $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x80); // 10000000
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }

    #[test]
    fn test_asl_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x40; // 01000000 (at 0x42 + 0x05)
        let program = vec![0x16, 0x42, 0x00]; // ASL $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x47], 0x80); // 10000000
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }

    #[test]
    fn test_asl_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x4242] = 0x40; // 01000000
        let program = vec![0x0E, 0x42, 0x42, 0x00]; // ASL $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4242], 0x80); // 10000000
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }

    #[test]
    fn test_asl_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x40; // 01000000 (at 0x4242 + 0x08)
        let program = vec![0x1E, 0x42, 0x42, 0x00]; // ASL $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x424A], 0x80); // 10000000
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }
}
