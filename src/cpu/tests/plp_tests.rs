#[cfg(test)]
mod plp_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_plp_basic() {
        let mut cpu = Cpu::new(Ram::new());
        // Setup stack with a status byte
        cpu.registers.sp = 0xFE;
        // Set status value 10111010 on stack
        // Bit 7: N=1, Bit 6: V=0, Bit 5: 1 (ignored), Bit 4: 1 (ignored)
        // Bit 3: D=1, Bit 2: I=0, Bit 1: Z=1, Bit 0: C=0
        cpu.memory.data[0x01FF] = 0xBA;

        let program = vec![0x28, 0x00]; // PLP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Stack pointer should be incremented
        assert_eq!(cpu.registers.sp, 0xFF);

        // Verify flags were set correctly
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.v, 0);
        assert_eq!(cpu.flags.d, 1);
        assert_eq!(cpu.flags.i, 0);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.c, 0);
    }

    #[test]
    fn test_plp_all_flags_set() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack with all flags set
        cpu.registers.sp = 0xFE;
        cpu.memory.data[0x01FF] = 0xFF; // All bits set

        let program = vec![0x28, 0x00]; // PLP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Verify all flags were set
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.v, 1);
        assert_eq!(cpu.flags.d, 1);
        assert_eq!(cpu.flags.i, 1);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.c, 1);
    }

    #[test]
    fn test_plp_all_flags_clear() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack with a status byte with only bits 4 and 5 set
        // (which get ignored by PLP)
        cpu.registers.sp = 0xFE;
        cpu.memory.data[0x01FF] = 0x30; // Only bits 4 and 5 set

        let program = vec![0x28, 0x00]; // PLP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Verify all flags were cleared
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.v, 0);
        assert_eq!(cpu.flags.d, 0);
        assert_eq!(cpu.flags.i, 0);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.c, 0);
    }

    #[test]
    fn test_php_plp_combination() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup initial flags
        cpu.registers.sp = 0xFF;
        cpu.flags.n = 1;
        cpu.flags.v = 0;
        cpu.flags.d = 1;
        cpu.flags.i = 0;
        cpu.flags.z = 1;
        cpu.flags.c = 0;

        // Change some flags, push to stack, reset flags, then pull from stack
        let program = vec![
            0x08, // PHP - Push status to stack
            0x38, // SEC - Set carry flag
            0x78, // SEI - Set interrupt flag
            0xB8, // CLV - Clear overflow flag
            0x28, // PLP - Pull status from stack
            0x00, // BRK
        ];

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Verify flags were restored to initial values
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.v, 0);
        assert_eq!(cpu.flags.d, 1);
        assert_eq!(cpu.flags.i, 0);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.c, 0);
    }
}
