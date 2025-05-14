#[cfg(test)]
mod stx_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_stx_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x42;
        let program = vec![0x86, 0x20, 0x00]; // STX $20, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x20], 0x42);
    }

    #[test]
    fn test_stx_zeropage_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x42;
        cpu.registers.y = 0x05;
        let program = vec![0x96, 0x20, 0x00]; // STX $20,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x25], 0x42); // 0x20 + 0x05 = 0x25
    }

    #[test]
    fn test_stx_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x42;
        let program = vec![0x8E, 0x00, 0x40, 0x00]; // STX $4000, BRK
        cpu.load_program(&program, 0x8000);
        cpu.endless_run();
        assert_eq!(cpu.memory.data[0x4000], 0x42);
    }
}
