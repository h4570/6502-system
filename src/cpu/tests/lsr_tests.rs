#[cfg(test)]
mod lsr_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_lsr_accumulator() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x41; // 01000001
        let program = vec![0x4A, 0x00]; // LSR A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x20); // 00100000
        assert_eq!(cpu.flags.c, 1); // Carry set (bit 0 was 1)
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }

    #[test]
    fn test_lsr_accumulator_zero() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x01; // 00000001
        let program = vec![0x4A, 0x00]; // LSR A, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x00); // 00000000
        assert_eq!(cpu.flags.c, 1); // Carry set (bit 0 was 1)
        assert_eq!(cpu.flags.z, 1); // Result is zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }

    #[test]
    fn test_lsr_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x42; // 01000010
        let program = vec![0x46, 0x42, 0x00]; // LSR $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x42], 0x21); // 00100001
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }

    #[test]
    fn test_lsr_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x42; // 01000010 (at 0x42 + 0x05)
        let program = vec![0x56, 0x42, 0x00]; // LSR $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x47], 0x21); // 00100001
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }

    #[test]
    fn test_lsr_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x4242] = 0x42; // 01000010
        let program = vec![0x4E, 0x42, 0x42, 0x00]; // LSR $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4242], 0x21); // 00100001
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }

    #[test]
    fn test_lsr_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x42; // 01000010 (at 0x4242 + 0x08)
        let program = vec![0x5E, 0x42, 0x42, 0x00]; // LSR $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x424A], 0x21); // 00100001
        assert_eq!(cpu.flags.c, 0); // No carry set
        assert_eq!(cpu.flags.z, 0); // Result not zero
        assert_eq!(cpu.flags.n, 0); // Negative flag always clear
    }
}
