#[cfg(test)]
mod cmp_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_cmp_immediate_equal() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        let program = vec![0xC9, 0x40, 0x00]; // CMP #$40, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.n, 0); // Result 0, so N=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_immediate_greater() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        let program = vec![0xC9, 0x30, 0x00]; // CMP #$30, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.n, 0); // Result positive, so N=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_immediate_less() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        let program = vec![0xC9, 0x50, 0x00]; // CMP #$50, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.n, 1); // Result negative, so N=1
        assert_eq!(cpu.flags.c, 0); // A < M, so C=0
    }

    #[test]
    fn test_cmp_zeropage() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.memory.data[0x42] = 0x40;
        let program = vec![0xC5, 0x42, 0x00]; // CMP $42, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_zeropage_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.registers.x = 0x05;
        cpu.memory.data[0x47] = 0x35; // 0x42 + 0x05 = 0x47
        let program = vec![0xD5, 0x42, 0x00]; // CMP $42,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.n, 0); // Result positive, so N=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_absolute() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.memory.data[0x4242] = 0x40;
        let program = vec![0xCD, 0x42, 0x42, 0x00]; // CMP $4242, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 1); // Equal, so Z=1
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_absolute_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.registers.x = 0x08;
        cpu.memory.data[0x424A] = 0x35; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xDD, 0x42, 0x42, 0x00]; // CMP $4242,X, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_absolute_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.registers.y = 0x08;
        cpu.memory.data[0x424A] = 0x35; // 0x4242 + 0x08 = 0x424A
        let program = vec![0xD9, 0x42, 0x42, 0x00]; // CMP $4242,Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_indirect_x() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.registers.x = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x20] = 0x42; // 0x1C + 0x04 = 0x20
        cpu.memory.data[0x21] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3742] = 0x35;
        let program = vec![0xC1, 0x1C, 0x00]; // CMP ($1C,X), BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }

    #[test]
    fn test_cmp_indirect_y() {
        let mut cpu = Cpu::new(Ram::new());
        cpu.registers.a = 0x40;
        cpu.registers.y = 0x04;
        // Set up the indirect pointer
        cpu.memory.data[0x40] = 0x42;
        cpu.memory.data[0x41] = 0x37;
        // Set up the target data
        cpu.memory.data[0x3746] = 0x35; // 0x3742 + 0x04 = 0x3746
        let program = vec![0xD1, 0x40, 0x00]; // CMP ($40),Y, BRK
        cpu.load_program(&program, 0x8000);
        cpu.run();
        assert_eq!(cpu.flags.z, 0); // Not equal, so Z=0
        assert_eq!(cpu.flags.c, 1); // A >= M, so C=1
    }
}
