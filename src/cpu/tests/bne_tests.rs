#[cfg(test)]
mod bne_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_bne_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear zero flag
        cpu.flags.z = 0;
        // BNE +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0xD0, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_bne_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set zero flag
        cpu.flags.z = 1;
        // BNE +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0xD0, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_bne_simple_loop() {
        let mut cpu = Cpu::new(Ram::new());
        // LDX #$03, DEX, BNE -2, LDA #$42, BRK
        let program = vec![
            0xA2, 0x03, // LDX #$03
            0xCA, // DEX
            0xD0, 0xFD, // BNE -3 (branch back to DEX)
            0xA9, 0x42, // LDA #$42
            0x00, // BRK
        ];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.x, 0x00); // X should be decremented to 0
        assert_eq!(cpu.registers.a, 0x42); // We should exit the loop and load 0x42
    }

    #[test]
    fn test_bne_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear zero flag
        cpu.flags.z = 0;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BNE +20 (crosses page boundary), BRK
        let program = vec![0xD0, 0x14, 0x00];
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
