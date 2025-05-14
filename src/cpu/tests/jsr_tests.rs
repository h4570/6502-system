#[cfg(test)]
mod jsr_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_jsr() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Initialize stack pointer
        cpu.registers.sp = 0xFF;
        
        // Program: JSR to $4242, BRK at $4242
        let program = vec![0x20, 0x42, 0x42, 0xEA]; // JSR $4242, NOP
        cpu.load_program(&program, 0x8000);
        
        // Put a BRK at the subroutine address
        cpu.memory.data[0x4242] = 0x00; // BRK
        
        cpu.run();
        
        // Program counter should be at $4243 (after BRK)
        assert_eq!(cpu.registers.pc, 0x4243);
        
        // Check if return address was correctly pushed to the stack (should be $8002)
        // Return address is PC - 1, which points to the last byte of JSR instruction
        assert_eq!(cpu.memory.data[0x01FE], 0x02); // Low byte of return address
        assert_eq!(cpu.memory.data[0x01FF], 0x80); // High byte of return address
        
        // Check if stack pointer was decremented twice
        assert_eq!(cpu.registers.sp, 0xFD);
    }
    
    #[test]
    fn test_jsr_stack_wrap() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Set stack pointer near wrapping point
        cpu.registers.sp = 0x01;
        
        let program = vec![0x20, 0x42, 0x42, 0xEA]; // JSR $4242, NOP
        cpu.load_program(&program, 0x8000);
        
        // Put a BRK at the subroutine address
        cpu.memory.data[0x4242] = 0x00; // BRK
        
        cpu.run();
        
        // Check if return address was correctly pushed to the stack
        assert_eq!(cpu.memory.data[0x0100], 0x02); // Low byte
        assert_eq!(cpu.memory.data[0x0101], 0x80); // High byte
        
        // Stack pointer should wrap around
        assert_eq!(cpu.registers.sp, 0xFF);
    }
}
