#[cfg(test)]
mod ldx_tests {
    use crate::cpu::cpu::Cpu;

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = Cpu::new();
        let program = vec![0xA2, 0x42, 0x00]; // LDX #$42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_ldx_zero_flag() {
        let mut cpu = Cpu::new();
        let program = vec![0xA2, 0x00, 0x00]; // LDX #$00, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_ldx_negative_flag() {
        let mut cpu = Cpu::new();
        let program = vec![0xA2, 0x80, 0x00]; // LDX #$80, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_ldx_zeropage() {
        let mut cpu = Cpu::new();
        cpu.memory.data[0x42] = 0x37;
        let program = vec![0xA6, 0x42, 0x00]; // LDX $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x37);
    }

    #[test]
    fn test_ldx_absolute() {
        let mut cpu = Cpu::new();
        cpu.memory.data[0x4242] = 0x37;
        let program = vec![0xAE, 0x42, 0x42, 0x00]; // LDX $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x37);
    }

    #[test]
    fn test_ldx_zeropage_y() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0x05;
        cpu.memory.data[0x47] = 0x37; // 0x42 + 0x05 = 0x47
        let program = vec![0xB6, 0x42, 0x00]; // LDX $42,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x37);
    }

    #[test]
    fn test_ldx_absolute_y() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0x37; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xBE, 0x42, 0x42, 0x00]; // LDX $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x37);
    }

    #[test]
    fn test_ldx_absolute_y_page_cross() {
        let mut cpu = Cpu::new();
        cpu.registers.y = 0xFF;
        cpu.memory.data[0x4341] = 0x37; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xBE, 0x42, 0x42, 0x00]; // LDX $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.registers.x, 0x37);
    }
}
