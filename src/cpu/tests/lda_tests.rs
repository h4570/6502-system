#[cfg(test)]
mod lda_tests {
    use crate::cpu::cpu::Cpu;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();
        let program = vec![0xA9, 0x42, 0x00]; // LDA #$42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = Cpu::new();
        let program = vec![0xA9, 0x00, 0x00]; // LDA #$00, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = Cpu::new();
        let program = vec![0xA9, 0x80, 0x00]; // LDA #$80, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = Cpu::new();
        cpu.memory.data[0x42] = 0x37;
        let program = vec![0xA5, 0x42, 0x00]; // LDA $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu = Cpu::new();
        cpu.memory.data[0x4242] = 0x37;
        let program = vec![0xAD, 0x42, 0x42, 0x00]; // LDA $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_zeropage_x() {
        let mut cpu = Cpu::new();
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x37; // 0x42 + 0x05 = 0x47
        let program = vec![0xB5, 0x42, 0x00]; // LDA $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = Cpu::new();
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x37; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xBD, 0x42, 0x42, 0x00]; // LDA $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_absolute_x_page_cross() {
        let mut cpu = Cpu::new();
        cpu.registers.x = 0xFF;
        cpu.memory.data[0x4341] = 0x37; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xBD, 0x42, 0x42, 0x00]; // LDA $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0x37; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xB9, 0x42, 0x42, 0x00]; // LDA $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_absolute_y_page_cross() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0xFF;
        cpu.memory.data[0x4341] = 0x37; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xB9, 0x42, 0x42, 0x00]; // LDA $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x37);
    }

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = Cpu::new();
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x42; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3742] = 0x84;
        let program = vec![0xA1, 0x1C, 0x00]; // LDA ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x84);
    }

    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3746] = 0x84; // 0x3742 + 0x04 = 0x3746
        let program = vec![0xB1, 0x40, 0x00]; // LDA ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x84);
    }

    #[test]
    fn test_lda_indirect_y_page_cross() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0xFF;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3841] = 0x84; // 0x3742 + 0xFF = 0x3841 (page boundary crossed)
        let program = vec![0xB1, 0x40, 0x00]; // LDA ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x84);
    }
}
