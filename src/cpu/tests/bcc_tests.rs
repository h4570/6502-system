#[cfg(test)]
mod bcc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_bcc_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear carry flag
        cpu.flags.c = 0;
        // BCC +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0x90, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_bcc_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set carry flag
        cpu.flags.c = 1;
        // BCC +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0x90, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_bcc_addition_loop() {
        let mut cpu = Cpu::new(Ram::new());

        // Set up memory for our test
        cpu.memory.data[0x50] = 5; // Loop counter - we'll add 5 numbers
        cpu.memory.data[0x60] = 10; // First number to add
        cpu.memory.data[0x61] = 20; // Second number
        cpu.memory.data[0x62] = 30; // Third number
        cpu.memory.data[0x63] = 40; // Fourth number
        cpu.memory.data[0x64] = 200; // Fifth number (will cause carry)
        cpu.memory.data[0x70] = 0; // Result location

        // Program: Sum an array of values until carry occurs or loop completes
        let program = vec![
            0xA2, 0x00, // LDX #$00 (initialize index)
            0xA9, 0x00, // LDA #$00 (initialize accumulator)
            0x85, 0x70, // STA $70 (initialize result)
            // Loop start
            0x18, // CLC (clear carry flag)
            0xA5, 0x70, // LDA $70 (load current sum)
            0x7D, 0x60, 0x00, // ADC $0060,X (add indexed value)
            0x85, 0x70, // STA $70 (store result)
            0xE8, // INX (increment index)
            0xE4, 0x50, // CPX $50 (compare with counter)
            0xF0, 0x02, // BEQ +2 (exit if equal to counter)
            0x90, 0xF4, // BCC -12 (branch back if carry clear)
            0x00, // BRK (end program)
        ];

        cpu.load_program(&program, 0x8000);
        cpu.run();

        // 10 + 20 + 30 + 40 + 200 = 300 (0x12C), but 8-bit can only store 0x2C with carry set
        assert_eq!(cpu.memory.data[0x70], 0x2C); // Sum should be 300 mod 256 = 44
        assert_eq!(cpu.flags.c, 1); // Carry should be set after adding 200
        assert_eq!(cpu.registers.x, 5); // Should have processed all 5 numbers
    }

    #[test]
    fn test_bcc_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear carry flag
        cpu.flags.c = 0;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BCC +20 (crosses page boundary), BRK
        let program = vec![0x90, 0x14, 0x00];
        cpu.load_program(&program, program_addr);

        // Put a LDA #$42 at the target location
        let target_addr = 0x8106; // program_addr + 2 (current instruction) + 20 (offset)
        let target_program = vec![0xA9, 0x42, 0x00]; // LDA #$42, BRK
        cpu.load_program(&target_program, target_addr);

        cpu.registers.pc = program_addr;
        cpu.run();

        assert_eq!(cpu.registers.a, 0x42); // Verify we executed the instruction after the page boundary
    }
}
