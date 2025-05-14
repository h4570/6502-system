#[cfg(test)]
mod sec_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_sec() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Ensure carry flag is cleared initially
        cpu.flags.c = 0;
        
        let program = vec![0x38, 0x00]; // SEC, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        assert_eq!(cpu.flags.c, 1, "Carry flag should be set");
    }

    #[test]
    fn test_sec_does_not_affect_other_flags() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Set initial flag values
        cpu.flags.n = 1;
        cpu.flags.z = 1;
        cpu.flags.i = 1;
        cpu.flags.d = 1;
        cpu.flags.v = 1;
        cpu.flags.c = 0;
        
        let program = vec![0x38, 0x00]; // SEC, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Check carry is set
        assert_eq!(cpu.flags.c, 1, "Carry flag should be set");
        
        // Other flags should be unchanged
        assert_eq!(cpu.flags.n, 1, "Negative flag should be unchanged");
        assert_eq!(cpu.flags.z, 1, "Zero flag should be unchanged");
        assert_eq!(cpu.flags.i, 1, "Interrupt flag should be unchanged");
        assert_eq!(cpu.flags.d, 1, "Decimal flag should be unchanged");
        assert_eq!(cpu.flags.v, 1, "Overflow flag should be unchanged");
    }
}
