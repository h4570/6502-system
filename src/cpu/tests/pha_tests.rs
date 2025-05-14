#[cfg(test)]
mod pha_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_pha() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Initialize the accumulator with a value
        cpu.registers.a = 0x42;
        // Initialize stack pointer (0xFF is the default after reset)
        cpu.registers.sp = 0xFF;
        
        let program = vec![0x48, 0x00]; // PHA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Check if the value was correctly pushed to the stack
        assert_eq!(cpu.memory.data[0x01FF], 0x42); // Stack starts at 0x0100 + SP
        // Check if stack pointer was decremented
        assert_eq!(cpu.registers.sp, 0xFE);
    }

    #[test]
    fn test_pha_wrap_around() {
        let mut cpu = Cpu::new(Ram::new());
        
        cpu.registers.a = 0x37;
        // Set stack pointer to 0, which should wrap around to 0xFF after push
        cpu.registers.sp = 0x00;
        
        let program = vec![0x48, 0x00]; // PHA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Check if value was pushed correctly
        assert_eq!(cpu.memory.data[0x0100], 0x37);
        // Stack pointer should wrap around to 0xFF
        assert_eq!(cpu.registers.sp, 0xFF);
    }
}
