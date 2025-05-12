#[cfg(test)]
mod adc_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_adc_immediate() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        let program = vec![0x69, 0x20, 0x00]; // ADC #$20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x30);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.flags.v, 0);
    }

    #[test]
    fn test_adc_carry_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0xFF;
        let program = vec![0x69, 0x01, 0x00]; // ADC #$01, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.c, 1);
    }

    #[test]
    fn test_adc_with_carry_set() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.flags.c = 1;
        let program = vec![0x69, 0x20, 0x00]; // ADC #$20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x31); // 0x10 + 0x20 + 1 (carry) = 0x31
        assert_eq!(cpu.flags.c, 0);
    }

    #[test]
    fn test_adc_overflow_flag() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x7F; // 127 (max positive 8-bit signed number)
        let program = vec![0x69, 0x01, 0x00]; // ADC #$01, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x80); // 127 + 1 = -128 (overflow)
        assert_eq!(cpu.flags.v, 1);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_adc_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.memory.data[0x42] = 0x15;
        let program = vec![0x65, 0x42, 0x00]; // ADC $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x15; // 0x42 + 0x05 = 0x47
        let program = vec![0x75, 0x42, 0x00]; // ADC $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.memory.data[0x4242] = 0x15;
        let program = vec![0x6D, 0x42, 0x42, 0x00]; // ADC $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x15; // 0x4242 + 0x08 = 0x424A
        let program = vec![0x7D, 0x42, 0x42, 0x00]; // ADC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_absolute_x_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.x = 0xFF;
        cpu.memory.data[0x4341] = 0x15; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0x7D, 0x42, 0x42, 0x00]; // ADC $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_absolute_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0x15; // 0x4242 + 0x08 = 0x424A
        let program = vec![0x79, 0x42, 0x42, 0x00]; // ADC $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_absolute_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.y = 0xFF;
        cpu.memory.data[0x4341] = 0x15; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0x79, 0x42, 0x42, 0x00]; // ADC $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_indirect_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x42; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3742] = 0x15;
        let program = vec![0x61, 0x1C, 0x00]; // ADC ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_indirect_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3746] = 0x15; // 0x3742 + 0x04 = 0x3746
        let program = vec![0x71, 0x40, 0x00]; // ADC ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }

    #[test]
    fn test_adc_indirect_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x10;
        cpu.registers.y = 0xFF;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3841] = 0x15; // 0x3742 + 0xFF = 0x3841 (page boundary crossed)
        let program = vec![0x71, 0x40, 0x00]; // ADC ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.a, 0x25);
    }
}
