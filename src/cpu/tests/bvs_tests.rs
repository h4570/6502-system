#[cfg(test)]
mod bvs_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_bvs_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set overflow flag
        cpu.flags.v = 1;
        // BVS +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0x70, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_bvs_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear overflow flag
        cpu.flags.v = 0;
        // BVS +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0x70, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_bvs_detect_overflow() {
        let mut cpu = Cpu::new(Ram::new());

        // Program: Detect and handle overflow cases in a series of additions
        let program = vec![
            0xA9, 0x7F, // LDA #$7F (127)
            0x8D, 0x80, 0x00, // STA $0080 (store 127 to check later)
            0xA9, 0x01, // LDA #$01 (1)
            0x18, // CLC
            0x6D, 0x80, 0x00, // ADC $0080 (add 127, will cause overflow)
            0x70, 0x06, // BVS +6 (branch if overflow set) - Fixed: was +7
            0xA9, 0xFF, // LDA #$FF (shouldn't execute)
            0x8D, 0x81, 0x00, // STA $0081 (shouldn't execute)
            0x00, // BRK (shouldn't execute)
            // Overflow handler
            0xA9, 0x42, // LDA #$42 (66 - flag for overflow detected)
            0x8D, 0x81, 0x00, // STA $0081
            0x00, // BRK
        ];

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // Should have detected overflow when adding 127 + 1
        assert_eq!(cpu.flags.v, 1);
        assert_eq!(cpu.memory.data[0x81], 0x42); // Overflow was detected
    }

    #[test]
    fn test_bvs_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Set overflow flag
        cpu.flags.v = 1;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BVS +20 (crosses page boundary), BRK
        let program = vec![0x70, 0x14, 0x00];
        cpu.load_program(&program, program_addr);

        // Put a LDA #$42 at the target location
        let target_addr = 0x8106; // program_addr + 2 (current instruction) + 20 (offset)
        let target_program = vec![0xA9, 0x42, 0x00]; // LDA #$42, BRK
        cpu.load_program(&target_program, target_addr);

        cpu.registers.pc = program_addr;
        cpu.endless_run();

        assert_eq!(cpu.registers.a, 0x42); // Verify we executed the instruction after the page boundary
    }
}
