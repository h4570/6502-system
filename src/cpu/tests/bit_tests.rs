#[cfg(test)]
mod bit_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;
    #[test]
    fn test_bit_zeropage_result_non_zero() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010; // 170 (0xAA)
        cpu.memory.data[0x42] = 0b01010101; // 85 (0x55)
        let program = vec![0x24, 0x42, 0x00]; // BIT $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        // AND result is 0, but register A should not change
        assert_eq!(cpu.registers.a, 0xAA);
        // Result is zero
        assert_eq!(cpu.flags.z, 1);
        // Bit 7 (N) of memory is 0
        assert_eq!(cpu.flags.n, 0);
        // Bit 6 (V) of memory is 1 (7th bit from right in 01010101)
        assert_eq!(cpu.flags.v, 1);
    }

    #[test]
    fn test_bit_zeropage_negative_and_overflow() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b00000001; // 1
        cpu.memory.data[0x42] = 0b11000000; // 192 (0xC0)
        let program = vec![0x24, 0x42, 0x00]; // BIT $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        // AND result is 0, but register A should not change
        assert_eq!(cpu.registers.a, 0x01);
        // Result is zero
        assert_eq!(cpu.flags.z, 1);
        // Bit 7 (N) of memory is 1
        assert_eq!(cpu.flags.n, 1);
        // Bit 6 (V) of memory is 1
        assert_eq!(cpu.flags.v, 1);
    }

    #[test]
    fn test_bit_zeropage_non_zero_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b00000011; // 3
        cpu.memory.data[0x42] = 0b00000011; // 3
        let program = vec![0x24, 0x42, 0x00]; // BIT $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        // AND result is non-zero (3), but register A should not change
        assert_eq!(cpu.registers.a, 0x03);
        // Result is not zero
        assert_eq!(cpu.flags.z, 0);
        // Bit 7 (N) of memory is 0
        assert_eq!(cpu.flags.n, 0);
        // Bit 6 (V) of memory is 0
        assert_eq!(cpu.flags.v, 0);
    }

    #[test]
    fn test_bit_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010; // 170 (0xAA)
        cpu.memory.data[0x4242] = 0b11010101; // 213 (0xD5)
        let program = vec![0x2C, 0x42, 0x42, 0x00]; // BIT $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        // AND result is non-zero, but register A should not change
        assert_eq!(cpu.registers.a, 0xAA);
        // Result is not zero
        assert_eq!(cpu.flags.z, 0);
        // Bit 7 (N) of memory is 1
        assert_eq!(cpu.flags.n, 1);
        // Bit 6 (V) of memory is 1
        assert_eq!(cpu.flags.v, 1);
    }
}
