#[cfg(test)]
mod bpl_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_bpl_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear negative flag
        cpu.flags.n = 0;
        // BPL +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0x10, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_bpl_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set negative flag
        cpu.flags.n = 1;
        // BPL +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0x10, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_bpl_count_positive_values() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup memory with values
        cpu.memory.data[0x60] = 5; // Number of values to check
        cpu.memory.data[0x70] = 0; // Counter for positive values
        cpu.memory.data[0x80] = 10; // Positive
        cpu.memory.data[0x81] = 0; // Positive (zero)
        cpu.memory.data[0x82] = 0xFF; // Negative
        cpu.memory.data[0x83] = 0x7F; // Positive
        cpu.memory.data[0x84] = 0x80; // Negative

        // Program: Count positive values in a sequence
        let program = vec![
            0xA2, 0x00, // LDX #$00 (index)
            0xA9, 0x00, // LDA #$00 (counter)
            0x85, 0x70, // STA $70 (store counter)
            // Loop start
            0xBD, 0x80, 0x00, // LDA $0080,X (load array value)
            0x10, 0x04, // BPL +4 (branch if positive)
            0xE8, // INX (increment index)
            0x4C, 0x14, 0x80, // JMP $8014 (jump to compare)
            // Positive value code
            0xE6, 0x70, // INC $70 (increment positive counter)
            0xE8, // INX (increment index)
            // Compare index to length
            0xE4, 0x60, // CPX $60 (compare with array length)
            0x90, 0xF0, // BCC -16 (branch back if less)
            0x00, // BRK (end program)
        ];

        cpu.load_program(&program, 0x8000);
        cpu.run();

        // Should count 3 positive values (10, 0, and 127)
        assert_eq!(cpu.memory.data[0x70], 3);
    }

    #[test]
    fn test_bpl_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear negative flag
        cpu.flags.n = 0;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BPL +20 (crosses page boundary), BRK
        let program = vec![0x10, 0x14, 0x00];
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
