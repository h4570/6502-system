#[cfg(test)]
mod ldy_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x42, 0x00]; // LDY #$42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_ldy_zero_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x00, 0x00]; // LDY #$00, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.flags.z, 1);
        assert_eq!(cpu.flags.n, 0);
    }

    #[test]
    fn test_ldy_negative_flag() {
        let mut cpu = Cpu::new(Ram::new());
        let program = vec![0xA0, 0x80, 0x00]; // LDY #$80, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x80);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.n, 1);
    }

    #[test]
    fn test_ldy_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x42] = 0x37;
        let program = vec![0xA4, 0x42, 0x00]; // LDY $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x37);
    }

    #[test]
    fn test_ldy_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.memory.data[0x4242] = 0x37;
        let program = vec![0xAC, 0x42, 0x42, 0x00]; // LDY $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x37);
    }

    #[test]
    fn test_ldy_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x37; // 0x42 + 0x05 = 0x47
        let program = vec![0xB4, 0x42, 0x00]; // LDY $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x37);
    }

    #[test]
    fn test_ldy_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x37; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xBC, 0x42, 0x42, 0x00]; // LDY $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x37);
    }

    #[test]
    fn test_ldy_absolute_x_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0xFF;
        cpu.memory.data[0x4341] = 0x37; // 0x4242 + 0xFF = 0x4341 (page boundary crossed)
        let program = vec![0xBC, 0x42, 0x42, 0x00]; // LDY $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.registers.y, 0x37);
    }
}
