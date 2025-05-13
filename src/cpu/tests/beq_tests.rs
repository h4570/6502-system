#[cfg(test)]
mod beq_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_beq_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set zero flag
        cpu.flags.z = 1;
        // BEQ +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0xF0, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_beq_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear zero flag
        cpu.flags.z = 0;
        // BEQ +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0xF0, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_beq_negative_offset() {
        let mut cpu = Cpu::new(Ram::new());
        // LDA #$00, STA $50, LDA #$01, BEQ +2, LDA #$42, STA $51, LDA $50, BEQ -8, LDA #$24, BRK
        let program = vec![
            0xA9, 0x00, // LDA #$00
            0x85, 0x50, // STA $50
            0xA9, 0x01, // LDA #$01
            0xF0, 0x02, // BEQ +2 (should not branch)
            0xA9, 0x42, // LDA #$42
            0x85, 0x51, // STA $51
            0xA5, 0x50, // LDA $50 (load 0)
            0xF0, 0xF8, // BEQ -8 (should branch back to LDA #$01)
            0xA9, 0x24, // LDA #$24
            0x00, // BRK
        ];
        cpu.load_program(&program, 0x8000);
        cpu.run();
        // Since we're creating a loop, we rely on the max_iterations safety limit
        // We should have loaded and stored 0x42 to address 0x51 before the loop was broken
        assert_eq!(cpu.memory.data[0x51], 0x42);
    }

    #[test]
    fn test_beq_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Set zero flag
        cpu.flags.z = 1;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BEQ +20 (crosses page boundary), BRK
        let program = vec![0xF0, 0x14, 0x00];
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
