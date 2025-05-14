#[cfg(test)]
mod jmp_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = Cpu::new(Ram::new());

        // Program sets up a jump to $4242
        let program = vec![
            0x4C, 0x42, 0x42, // JMP $4242
            0xEA, 0xEA, // NOP instructions we should skip
        ];

        // Put a BRK at the target address
        cpu.memory.data[0x4242] = 0x00; // BRK

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // The PC should have jumped to $4242 and then increased by 1 after executing BRK
        assert_eq!(cpu.registers.pc, 0x4243);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = Cpu::new(Ram::new());

        // Set up the pointer at $3030
        cpu.memory.data[0x3030] = 0x42;
        cpu.memory.data[0x3031] = 0x42;

        // Program jumps to the address pointed to by $3030
        let program = vec![
            0x6C, 0x30, 0x30, // JMP ($3030)
            0xEA, 0xEA, // NOP instructions we should skip
        ];

        // Put a BRK at the target address
        cpu.memory.data[0x4242] = 0x00; // BRK

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // The PC should have jumped to $4242 and then increased by 1 after executing BRK
        assert_eq!(cpu.registers.pc, 0x4243);
    }

    #[test]
    fn test_jmp_indirect_page_boundary_bug() {
        let mut cpu = Cpu::new(Ram::new());

        // Set up the pointer at $30FF (page boundary)
        cpu.memory.data[0x30FF] = 0x42;
        cpu.memory.data[0x3000] = 0x42; // This should be the high byte (not $3100)

        // Program jumps to the address pointed to by $30FF
        let program = vec![
            0x6C, 0xFF, 0x30, // JMP ($30FF)
            0xEA, 0xEA, // NOP instructions we should skip
        ];

        // Put a BRK at the target address
        cpu.memory.data[0x4242] = 0x00; // BRK

        cpu.load_program(&program, 0x8000);
        cpu.endless_run();

        // The PC should have jumped to $4242 and then increased by 1 after executing BRK
        assert_eq!(cpu.registers.pc, 0x4243);
    }

    #[test]
    fn test_jmp_to_infinite_loop() {
        let mut cpu = Cpu::new(Ram::new());

        // Set up an exit flag to break out of the infinite loop after a few iterations
        let mut iteration_count = 0;

        // Setup a program with a jump to itself (infinite loop)
        let program = vec![
            0x4C, 0x00, 0x80, // JMP $8000 (infinite loop)
        ];

        cpu.load_program(&program, 0x8000);

        // Use a custom run function that counts iterations
        while !cpu.exit && iteration_count < 5 {
            let opcode = cpu.fetch_byte();
            let instruction = cpu.instructions[opcode as usize];
            instruction(&mut cpu);

            // Check if PC has changed properly
            assert_eq!(cpu.registers.pc, 0x8000);

            // Count iterations and break out of the test loop after a few
            iteration_count += 1;
        }

        // Ensure we actually went through a few iterations
        assert_eq!(iteration_count, 5);
    }
}
