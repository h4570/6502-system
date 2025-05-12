#[cfg(test)]
mod nop_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Save original register and flag values
        let initial_a = 0x42;
        let initial_x = 0x24;
        let initial_y = 0x84;
        
        // Set up initial state
        cpu.registers.a = initial_a;
        cpu.registers.x = initial_x;
        cpu.registers.y = initial_y;
        
        // Execute NOP and BRK
        let program = vec![0xEA, 0x00]; // NOP, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Verify that NOP didn't change any registers or flags
        assert_eq!(cpu.registers.a, initial_a);
        assert_eq!(cpu.registers.x, initial_x);
        assert_eq!(cpu.registers.y, initial_y);
    }

    #[test]
    fn test_multiple_nops() {
        let mut cpu = Cpu::new(Ram::new());
        
        // Set up initial state
        cpu.registers.a = 0x42;
        
        // Execute 5 NOPs and BRK
        let program = vec![0xEA, 0xEA, 0xEA, 0xEA, 0xEA, 0x00]; // 5 NOPs, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        
        // Verify that NOPs didn't change the A register
        assert_eq!(cpu.registers.a, 0x42);
    }
}
