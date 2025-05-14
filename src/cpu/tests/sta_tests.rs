#[cfg(test)]
mod sta_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_sta_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        let program = vec![0x85, 0x20, 0x00]; // STA $20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x20], 0x42);
    }

    #[test]
    fn test_sta_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.x = 0x05;
        let program = vec![0x95, 0x20, 0x00]; // STA $20,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x25], 0x42); // 0x20 + 0x05 = 0x25
    }

    #[test]
    fn test_sta_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        let program = vec![0x8D, 0x00, 0x40, 0x00]; // STA $4000, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4000], 0x42);
    }

    #[test]
    fn test_sta_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.x = 0x08;
        let program = vec![0x9D, 0x00, 0x40, 0x00]; // STA $4000,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4008], 0x42); // 0x4000 + 0x08 = 0x4008
    }

    #[test]
    fn test_sta_absolute_x_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.x = 0xFF;
        let program = vec![0x9D, 0x00, 0x40, 0x00]; // STA $4000,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x40FF], 0x42); // 0x4000 + 0xFF = 0x40FF
    }

    #[test]
    fn test_sta_absolute_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.y = 0x08;
        let program = vec![0x99, 0x00, 0x40, 0x00]; // STA $4000,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4008], 0x42); // 0x4000 + 0x08 = 0x4008
    }

    #[test]
    fn test_sta_absolute_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.y = 0xFF;
        let program = vec![0x99, 0x00, 0x40, 0x00]; // STA $4000,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x40FF], 0x42); // 0x4000 + 0xFF = 0x40FF
    }

    #[test]
    fn test_sta_indirect_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x00; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x40;
        let program = vec![0x81, 0x1C, 0x00]; // STA ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4000], 0x42);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x00;
        cpu.memory.data[0x41] = 0x40;
        let program = vec![0x91, 0x40, 0x00]; // STA ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4004], 0x42); // 0x4000 + 0x04 = 0x4004
    }

    #[test]
    fn test_sta_indirect_y_page_cross() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x42;
        cpu.registers.y = 0xFF;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x00;
        cpu.memory.data[0x41] = 0x40;
        let program = vec![0x91, 0x40, 0x00]; // STA ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x40FF], 0x42); // 0x4000 + 0xFF = 0x40FF
    }
}
