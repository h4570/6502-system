#[cfg(test)]
mod cpx_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_cpx_immediate_equal() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x40;
        let program = vec![0xE0, 0x40, 0x00]; // CPX #$40, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.n, 0); // Result 0, so N=0
        assert_eq!(cpu.flags.c, 1); // X >= M, so C=1
    }

    #[test]
    fn test_cpx_immediate_greater() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x40;
        let program = vec![0xE0, 0x30, 0x00]; // CPX #$30, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.n, 0); // Result positive, so N=0
        assert_eq!(cpu.flags.c, 1); // X >= M, so C=1
    }

    #[test]
    fn test_cpx_immediate_less() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x40;
        let program = vec![0xE0, 0x50, 0x00]; // CPX #$50, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.n, 1); // Result negative, so N=1
        assert_eq!(cpu.flags.c, 0); // X < M, so C=0
    }

    #[test]
    fn test_cpx_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x40;
        cpu.memory.data[0x42] = 0x40;
        let program = vec![0xE4, 0x42, 0x00]; // CPX $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.c, 1); // X >= M, so C=1
    }

    #[test]
    fn test_cpx_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.x = 0x40;
        cpu.memory.data[0x4242] = 0x40;
        let program = vec![0xEC, 0x42, 0x42, 0x00]; // CPX $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.c, 1); // X >= M, so C=1
    }
}
