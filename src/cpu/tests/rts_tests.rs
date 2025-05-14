#[cfg(test)]
mod rts_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_jsr_and_rts() {
        let mut cpu = Cpu::new(Ram::new());

        // Initialize stack pointer
        cpu.registers.sp = 0xFF;

        // Program:
        // 8000: JSR $8010
        // 8003: NOP          ; We should return here after RTS
        // 8004: BRK
        // 8010: NOP          ; Subroutine
        // 8011: RTS

        let mut program = vec![0; 0x20]; // Create a larger space for the program
        program[0] = 0x20; // JSR $8010
        program[1] = 0x10; // Low byte of address
        program[2] = 0x80; // High byte of address
        program[3] = 0xEA; // NOP (we should return here)
        program[4] = 0x00; // BRK

        // Subroutine at $8010
        program[0x10] = 0xEA; // NOP
        program[0x11] = 0x60; // RTS

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Check that we executed both the subroutine and made it back to the NOP
        // After the NOP, we should have hit the BRK, so PC should be at $8005
        assert_eq!(cpu.registers.pc, 0x8005);
    }
    #[test]
    fn test_rts_stack_manipulation() {
        let mut cpu = Cpu::new(Ram::new());

        // Simulate stack state after JSR
        cpu.registers.sp = 0xFD;
        cpu.memory.data[0x01FE] = 0x42; // Low byte of return address
        cpu.memory.data[0x01FF] = 0x37; // High byte of return address

        // Program: RTS only (removing BRK)
        let program = vec![0x60];
        cpu.load_program(&program, 0x8000);

        // We'll manually handle the execution to check state immediately after RTS
        let opcode = cpu.fetch_byte();
        let instruction = cpu.instructions[opcode as usize];
        instruction(&mut cpu);

        // PC should be at return address + 1
        assert_eq!(cpu.registers.pc, 0x3743);

        // Stack pointer should be restored
        assert_eq!(cpu.registers.sp, 0xFF);
    }
    #[test]
    fn test_rts_stack_wrap() {
        let mut cpu = Cpu::new(Ram::new());

        // Set stack pointer near wrapping point
        cpu.registers.sp = 0xFE;
        cpu.memory.data[0x01FF] = 0x42; // Low byte of return address
        cpu.memory.data[0x0100] = 0x37; // High byte of return address

        // Program: RTS only (removing BRK)
        let program = vec![0x60];
        cpu.load_program(&program, 0x8000);

        // We'll manually handle the execution to check state immediately after RTS
        let opcode = cpu.fetch_byte();
        let instruction = cpu.instructions[opcode as usize];
        instruction(&mut cpu);

        // PC should be at return address + 1
        assert_eq!(cpu.registers.pc, 0x3743);

        // Stack pointer should be correct after pulling two bytes
        assert_eq!(cpu.registers.sp, 0x00);
    }
}
