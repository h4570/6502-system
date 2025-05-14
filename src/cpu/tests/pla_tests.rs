#[cfg(test)]
mod pla_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_pla() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Initialize stack pointer
        cpu.registers.sp = 0xFE;
        // Put a value on the stack
        cpu.memory.data[0x01FF] = 0x42;
        
        let program = vec![0x68, 0x00]; // PLA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Check if accumulator has the pulled value
        assert_eq!(cpu.registers.a, 0x42);
        // Check if stack pointer was incremented
        assert_eq!(cpu.registers.sp, 0xFF);
        // Zero flag should be clear
        assert_eq!(cpu.flags.z, 0);
        // Negative flag should be clear
        assert_eq!(cpu.flags.n, 0);
    }
    
    #[test]
    fn test_pla_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        
        cpu.registers.sp = 0xFE;
        cpu.memory.data[0x01FF] = 0x00; // Zero value on stack
        
        let program = vec![0x68, 0x00]; // PLA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        assert_eq!(cpu.registers.a, 0x00);
        // Zero flag should be set
        assert_eq!(cpu.flags.z, 1);
        // Negative flag should be clear
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_pla_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        
        cpu.registers.sp = 0xFE;
        cpu.memory.data[0x01FF] = 0x80; // Negative value (bit 7 set)
        
        let program = vec![0x68, 0x00]; // PLA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        assert_eq!(cpu.registers.a, 0x80);
        // Zero flag should be clear
        assert_eq!(cpu.flags.z, 0);
        // Negative flag should be set
        assert_eq!(cpu.flags.n, 1);
    }
    
    #[test]
    fn test_pla_wrap_around() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Set stack pointer to 0xFF, which should wrap around after pull
        cpu.registers.sp = 0xFF;
        cpu.memory.data[0x0100] = 0x37;
        
        let program = vec![0x68, 0x00]; // PLA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        assert_eq!(cpu.registers.a, 0x37);
        // Stack pointer should be 0x00 after increment from 0xFF
        assert_eq!(cpu.registers.sp, 0x00);
    }
}
