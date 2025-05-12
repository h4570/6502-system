#[cfg(test)]
mod sbc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_sbc_immediate() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1; // Set carry initially
        let program = vec![0xE9, 0x20, 0x00]; // SBC #$20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.c, 1); // No borrow occurred
        assert_eq!(cpu.flags.v, 0);
    }

    #[test]
    fn test_sbc_borrow() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x20;
        cpu.flags.c = 1; // Set carry initially (no borrow)
        let program = vec![0xE9, 0x30, 0x00]; // SBC #$30, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0xF0); // Result is -16 in two's complement
        assert_eq!(cpu.flags.n, 1); // Negative flag set
        assert_eq!(cpu.flags.c, 0); // Borrow occurred
    }

    #[test]
    fn test_sbc_without_carry() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 0; // Clear carry (borrow pending)
        let program = vec![0xE9, 0x20, 0x00]; // SBC #$20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x2F); // 0x50 - 0x20 - 1 = 0x2F
    }

    #[test]
    fn test_sbc_zero_result() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x20;
        cpu.flags.c = 1; // Set carry initially
        let program = vec![0xE9, 0x20, 0x00]; // SBC #$20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.z, 1); // Zero flag set
        assert_eq!(cpu.flags.c, 1); // No borrow
    }

    #[test]
    fn test_sbc_overflow_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x80; // -128 (min negative 8-bit signed number)
        cpu.flags.c = 1;
        let program = vec![0xE9, 0xFF, 0x00]; // SBC #$FF, BRK (subtracting -1)
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x81); // -128 - (-1) = -127
        assert_eq!(cpu.flags.v, 0); // No overflow since result (-127) is in valid range
        assert_eq!(cpu.flags.n, 1); // Negative flag set
    }

    #[test]
    fn test_sbc_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.memory.data[0x42] = 0x20;
        let program = vec![0xE5, 0x42, 0x00]; // SBC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x20; // 0x42 + 0x05 = 0x47
        let program = vec![0xF5, 0x42, 0x00]; // SBC $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.memory.data[0x4242] = 0x20;
        let program = vec![0xED, 0x42, 0x42, 0x00]; // SBC $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x20; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xFD, 0x42, 0x42, 0x00]; // SBC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_absolute_x_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.x = 0xFF;
        cpu.memory.data[0x4341] = 0x20; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xFD, 0x42, 0x42, 0x00]; // SBC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_absolute_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0x20; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xF9, 0x42, 0x42, 0x00]; // SBC $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_absolute_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.y = 0xFF;
        cpu.memory.data[0x4341] = 0x20; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xF9, 0x42, 0x42, 0x00]; // SBC $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_indirect_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x42; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3742] = 0x20;
        let program = vec![0xE1, 0x1C, 0x00]; // SBC ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_indirect_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3746] = 0x20; // 0x3742 + 0x04 = 0x3746
        let program = vec![0xF1, 0x40, 0x00]; // SBC ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }

    #[test]
    fn test_sbc_indirect_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x50;
        cpu.flags.c = 1;
        cpu.registers.y = 0xFF;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3841] = 0x20; // 0x3742 + 0xFF = 0x3841 (page boundary crossed)
        let program = vec![0xF1, 0x40, 0x00]; // SBC ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
    }
}
