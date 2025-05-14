#[cfg(test)]
mod eor_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_eor_immediate() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        let program = vec![0x49, 0b01010101, 0x00]; // EOR #$55, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b11111111);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_eor_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        let program = vec![0x49, 0b10101010, 0x00]; // EOR #$AA, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b00000000);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_eor_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.memory.data[0x42] = 0b01010101;
        let program = vec![0x45, 0x42, 0x00]; // EOR $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b11111111);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_eor_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0b01010101; // 0x42 + 0x05 = 0x47
        let program = vec![0x55, 0x42, 0x00]; // EOR $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b11111111);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_eor_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.memory.data[0x4242] = 0b11110000;
        let program = vec![0x4D, 0x42, 0x42, 0x00]; // EOR $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_eor_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0b11110000; // 0x4242 + 0x08 = 0x424A
        let program = vec![0x5D, 0x42, 0x42, 0x00]; // EOR $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_eor_absolute_x_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.x = 0xFF;
        cpu.memory.data[0x4341] = 0b11110000; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0x5D, 0x42, 0x42, 0x00]; // EOR $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }

    #[test]
    fn test_eor_absolute_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0b11110000; // 0x4242 + 0x08 = 0x424A
        let program = vec![0x59, 0x42, 0x42, 0x00]; // EOR $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }

    #[test]
    fn test_eor_absolute_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.y = 0xFF;
        cpu.memory.data[0x4341] = 0b11110000; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0x59, 0x42, 0x42, 0x00]; // EOR $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }

    #[test]
    fn test_eor_indirect_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x42; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3742] = 0b11110000;
        let program = vec![0x41, 0x1C, 0x00]; // EOR ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }

    #[test]
    fn test_eor_indirect_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3746] = 0b11110000; // 0x3742 + 0x04 = 0x3746
        let program = vec![0x51, 0x40, 0x00]; // EOR ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }

    #[test]
    fn test_eor_indirect_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0b10101010;
        cpu.registers.y = 0xFF;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3841] = 0b11110000; // 0x3742 + 0xFF = 0x3841 (page boundary crossed)
        let program = vec![0x51, 0x40, 0x00]; // EOR ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.a, 0b01011010);
    }
}
