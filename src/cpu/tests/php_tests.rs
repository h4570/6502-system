#[cfg(test)]
mod php_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_php_basic() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack pointer and flags
        cpu.registers.sp = 0xFF;
        cpu.flags.n = 1;
        cpu.flags.v = 0;
        cpu.flags.d = 1;
        cpu.flags.i = 0;
        cpu.flags.z = 1;
        cpu.flags.c = 0;

        let program = vec![0x08, 0x00]; // PHP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Stack pointer should be decremented
        assert_eq!(cpu.registers.sp, 0xFE);
        // Check the value on the stack
        // The value should be 10111010 = 0xBA
        // Bit 7: N=1, Bit 6: V=0, Bit 5: 1 (always set), Bit 4: 1 (always set)
        // Bit 3: D=1, Bit 2: I=0, Bit 1: Z=1, Bit 0: C=0
        assert_eq!(cpu.memory.data[0x01FF], 0xBA);
    }

    #[test]
    fn test_php_all_flags_set() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack pointer and flags
        cpu.registers.sp = 0xFF;
        cpu.flags.n = 1;
        cpu.flags.v = 1;
        cpu.flags.d = 1;
        cpu.flags.i = 1;
        cpu.flags.z = 1;
        cpu.flags.c = 1;

        let program = vec![0x08, 0x00]; // PHP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Check the value on the stack
        // The value should be 11111111 = 0xFF
        assert_eq!(cpu.memory.data[0x01FF], 0xFF);
    }

    #[test]
    fn test_php_all_flags_clear() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack pointer and flags
        cpu.registers.sp = 0xFF;
        cpu.flags.n = 0;
        cpu.flags.v = 0;
        cpu.flags.d = 0;
        cpu.flags.i = 0;
        cpu.flags.z = 0;
        cpu.flags.c = 0;

        let program = vec![0x08, 0x00]; // PHP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Check the value on the stack
        // The value should be 00110000 = 0x30 (only bits 4 and 5 are set)
        assert_eq!(cpu.memory.data[0x01FF], 0x30);
    }
}
