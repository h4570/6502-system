use crate::cpu::cpu::Cpu;

pub(crate) fn rti_0x40(cpu: &mut Cpu) -> u8 {
    // Debug - Print stack pointer before operation
    println!("RTI - Before operation: SP={:#04x}", cpu.registers.sp);

    // On the 6502, the stack grows downward, and SP points to the next free location
    // When pulling from stack, we first increment SP, then read from the location it now points to
    // Pull status register from stack
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let status = cpu.memory.data[0x0100 + cpu.registers.sp as usize];

    println!(
        "RTI - Status byte pulled: {:#04x} (binary: {:08b}) from stack address: {:#06x}",
        status,
        status,
        0x0100 + cpu.registers.sp as usize
    );
    // The status register is parsed according to the 6502 flag layout:
    // [N][V][1][B][D][I][Z][C] (where B is ignored during RTI)
    let n_flag = (status >> 7) & 1;
    let v_flag = (status >> 6) & 1;
    // Bit 5 is always set in stored status
    // Bit 4 (B flag) is ignored in RTI
    let d_flag = (status >> 3) & 1;
    let i_flag = (status >> 2) & 1;
    let z_flag = (status >> 1) & 1;
    let c_flag = status & 1; // Debug individual bits
    println!(
        "RTI - Flag bits from status byte {:#04x}: N={} V={} D={} I={} Z={} C={}",
        status, n_flag, v_flag, d_flag, i_flag, z_flag, c_flag
    );

    cpu.flags.n = n_flag; // Bit 7: Negative
    cpu.flags.v = v_flag; // Bit 6: Overflow
    cpu.flags.d = d_flag; // Bit 3: Decimal
    cpu.flags.i = i_flag; // Bit 2: Interrupt disable
    cpu.flags.z = z_flag; // Bit 1: Zero
    cpu.flags.c = c_flag; // Bit 0: Carry

    println!(
        "RTI - Flags set: N={} V={} D={} I={} Z={} C={}",
        cpu.flags.n, cpu.flags.v, cpu.flags.d, cpu.flags.i, cpu.flags.z, cpu.flags.c
    );

    // Pull program counter from stack - low byte first, then high byte
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let pc_lo = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;
    println!(
        "RTI - PC low byte: {:#04x} from stack address: {:#06x}",
        pc_lo,
        0x0100 + cpu.registers.sp as usize
    );

    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let pc_hi = cpu.memory.data[0x0100 + cpu.registers.sp as usize] as u16;
    println!(
        "RTI - PC high byte: {:#04x} from stack address: {:#06x}",
        pc_hi,
        0x0100 + cpu.registers.sp as usize
    ); // Set program counter - this is the address we'll return to
    // Important: The PC needs to be set to the exact address, not PC+1
    // For RTI, the return address is the exact address to execute from
    let return_addr = (pc_hi << 8) | pc_lo;
    println!(
        "RTI - Return address calculated: {:#06x} (hi:{:#04x}, lo:{:#04x})",
        return_addr, pc_hi, pc_lo
    );

    cpu.registers.pc = return_addr;

    // Adjust PC to compensate for the fact that the CPU's fetch_byte will increment it again
    // after this instruction completes but before executing the next instruction
    cpu.registers.pc = cpu.registers.pc.wrapping_sub(1);

    println!(
        "RTI - PC set to: {:#06x} (adjusted for fetch_byte), SP now: {:#04x}",
        cpu.registers.pc, cpu.registers.sp
    );

    println!("RTI[0x40] Implicit - Complete");
    6
}
