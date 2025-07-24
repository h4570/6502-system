#[cfg(test)]
mod rti_tests {
    use crate::cpu::cpu::Cpu;
    use crate::mem::ram::Ram;
    use log::debug;

    #[test]
    fn test_rti_basic() {
        let mut cpu = Cpu::new(Ram::new());
        // Setup stack with return address and status
        cpu.registers.sp = 0xFC; // Leave space for 3 bytes
        // Status register value to restore (with bit 5 set)
        // [N][V][1][B][D][I][Z][C]
        let status_value = 0b00111101; // N=0, V=0, bit5=1, B=1, D=1, I=1, Z=0, C=1
        debug!("Status byte in binary: {:08b}", status_value);
        debug!(
            "Status flags expected:  N={}, V={}, 5={}, B={}, D={}, I={}, Z={}, C={}",
            (status_value >> 7) & 1,
            (status_value >> 6) & 1,
            (status_value >> 5) & 1,
            (status_value >> 4) & 1,
            (status_value >> 3) & 1,
            (status_value >> 2) & 1,
            (status_value >> 1) & 1,
            status_value & 1
        );

        // Return address (0x8420)
        let return_addr_hi = 0x84;
        let return_addr_lo = 0x20;

        // Push status and return address onto stack
        cpu.memory.data[0x01FD] = status_value;
        cpu.memory.data[0x01FE] = return_addr_lo;
        cpu.memory.data[0x01FF] = return_addr_hi;

        // Print initial values for debugging
        debug!("Initial SP: {:#04x}", cpu.registers.sp);
        debug!(
            "Initial Stack: {:#04x}, {:#04x}, {:#04x} at addresses 0x01FD, 0x01FE, 0x01FF",
            cpu.memory.data[0x01FD], cpu.memory.data[0x01FE], cpu.memory.data[0x01FF]
        );

        // Create a program with RTI
        let program = vec![0x40]; // RTI
        cpu.load_program(&program, 0x8000);
        debug!("PC before run: {:#06x}", cpu.registers.pc);
        cpu.endless_run();
        // Verify CPU state after RTI
        debug!("Expected PC: 0x8421, Actual PC: {:#06x}", cpu.registers.pc);
        debug!(
            "Expected test value: {:#04x} (binary: {:08b})",
            status_value, status_value
        );
        debug!(
            "Actual flags after RTI: N={} V={} D={} I={} Z={} C={}",
            cpu.flags.n, cpu.flags.v, cpu.flags.d, cpu.flags.i, cpu.flags.z, cpu.flags.c
        );
        assert_eq!(cpu.registers.pc, 0x8421);
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.v, 0);
        assert_eq!(cpu.flags.d, 1);
        assert_eq!(cpu.flags.i, 1);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.registers.sp, 0xFF);
    }

    #[test]
    fn test_rti_all_flags() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack with return address and status
        cpu.registers.sp = 0xFC; // Leave space for 3 bytes

        // Status register with all processor flags set
        let status_value = 0b11111101; // N=1, V=1, B ignored, bit5=1, D=1, I=1, Z=0, C=1

        // Return address (0x8420)
        let return_addr_hi = 0x84;
        let return_addr_lo = 0x20;

        // Push status and return address onto stack
        cpu.memory.data[0x01FD] = status_value;
        cpu.memory.data[0x01FE] = return_addr_lo;
        cpu.memory.data[0x01FF] = return_addr_hi;

        // Print initial values for debugging
        debug!("Initial SP: {:#04x}", cpu.registers.sp);
        debug!(
            "Initial Stack: {:#04x}, {:#04x}, {:#04x} at addresses 0x01FD, 0x01FE, 0x01FF",
            cpu.memory.data[0x01FD], cpu.memory.data[0x01FE], cpu.memory.data[0x01FF]
        );

        // Create a program with RTI
        let program = vec![0x40]; // RTI
        cpu.load_program(&program, 0x8000);
        debug!("PC before run: {:#06x}", cpu.registers.pc);
        cpu.endless_run();

        // Calculate what the status byte should be after RTI
        let calculated_status = ((cpu.flags.n as u8) << 7) |
            ((cpu.flags.v as u8) << 6) |
            (1 << 5) | // Bit 5 is always set
            (0 << 4) | // B flag is cleared
            ((cpu.flags.d as u8) << 3) |
            ((cpu.flags.i as u8) << 2) |
            ((cpu.flags.z as u8) << 1) |
            (cpu.flags.c as u8);

        debug!(
            "Flag values: N={}, V={}, D={}, I={}, Z={}, C={}",
            cpu.flags.n, cpu.flags.v, cpu.flags.d, cpu.flags.i, cpu.flags.z, cpu.flags.c
        );
        debug!(
            "Calculated status: {:#04x} (binary: {:08b})",
            calculated_status, calculated_status
        );

        // Verify CPU state after RTI
        assert_eq!(cpu.registers.pc, 0x8421);
        assert_eq!(cpu.flags.n, 1);
        assert_eq!(cpu.flags.v, 1);
        assert_eq!(cpu.flags.d, 1);
        assert_eq!(cpu.flags.i, 1);
        assert_eq!(cpu.flags.z, 0);
        assert_eq!(cpu.flags.c, 1);
        assert_eq!(cpu.registers.sp, 0xFF);
    }

    #[test]
    fn test_rti_in_sequence() {
        let mut cpu = Cpu::new(Ram::new());

        // Setup stack with return address and status
        cpu.registers.sp = 0xFC; // Leave space for 3 bytes

        // Status value with Z flag set
        let status_value = 0x32; // 00110010 - Z flag is set (bit 1)

        // Return address (0x8420)
        let return_addr_hi = 0x84;
        let return_addr_lo = 0x20;

        // Push status and return address onto stack
        cpu.memory.data[0x01FD] = status_value;
        cpu.memory.data[0x01FE] = return_addr_lo;
        cpu.memory.data[0x01FF] = return_addr_hi;

        // Print initial values for debugging
        debug!("Initial SP: {:#04x}", cpu.registers.sp);
        debug!(
            "Initial Stack: {:#04x}, {:#04x}, {:#04x} at addresses 0x01FD, 0x01FE, 0x01FF",
            cpu.memory.data[0x01FD], cpu.memory.data[0x01FE], cpu.memory.data[0x01FF]
        );

        // Create a program with RTI
        let program = vec![0x40]; // RTI
        cpu.load_program(&program, 0x8000);
        debug!("PC before run: {:#06x}", cpu.registers.pc);

        // Run the program
        cpu.endless_run();

        // Debug output
        debug!("Expected PC: 0x8421, Actual PC: {:#06x}", cpu.registers.pc);
        debug!(
            "Flag values: N={}, V={}, D={}, I={}, Z={}, C={}",
            cpu.flags.n, cpu.flags.v, cpu.flags.d, cpu.flags.i, cpu.flags.z, cpu.flags.c
        );

        // Check if RTI correctly restored the PC and flags
        assert_eq!(cpu.registers.pc, 0x8421); // Returned to correct address
        assert_eq!(cpu.flags.n, 0);
        assert_eq!(cpu.flags.v, 0);
        assert_eq!(cpu.flags.d, 0);
        assert_eq!(cpu.flags.i, 0); // I flag should be 0 from status byte 0x32
        assert_eq!(cpu.flags.z, 1); // Z flag should be set from status byte 0x32
        assert_eq!(cpu.flags.c, 0);
        assert_eq!(cpu.registers.sp, 0xFF);
    }
}
