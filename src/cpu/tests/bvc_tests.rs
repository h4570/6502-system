#[cfg(test)]
mod bvc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_bvc_branch_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear overflow flag
        cpu.flags.v = 0;
        // BVC +2, NOP, NOP, LDA #$42, BRK
        let program = vec![0x50, 0x02, 0xEA, 0xEA, 0xA9, 0x42, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we branched to LDA instruction
    }

    #[test]
    fn test_bvc_branch_not_taken() {
        let mut cpu = Cpu::new(Ram::new());
        // Set overflow flag
        cpu.flags.v = 1;
        // BVC +3, LDA #$42, BRK, NOP, LDA #$24, BRK
        let program = vec![0x50, 0x03, 0xA9, 0x42, 0x00, 0xEA, 0xA9, 0x24, 0x00];
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0x42); // Check that we didn't branch
    }

    #[test]
    fn test_bvc_page_crossing() {
        let mut cpu = Cpu::new(Ram::new());
        // Clear overflow flag
        cpu.flags.v = 0;

        // Set up a program at the end of a page
        let program_addr = 0x80F0; // Near the end of a page
        // BVC +20 (crosses page boundary), BRK
        let program = vec![0x50, 0x14, 0x00];
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
