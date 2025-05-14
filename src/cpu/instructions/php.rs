use crate::cpu::cpu::Cpu;
use log::trace;

pub(crate) fn php_0x08(cpu: &mut Cpu) -> u8 {
    trace!("PHP[0x08]");

    // Debug the flag values before any calculation
    println!(
        "Flag values: N={}, V={}, D={}, I={}, Z={}, C={}",
        cpu.flags.n, cpu.flags.v, cpu.flags.d, cpu.flags.i, cpu.flags.z, cpu.flags.c
    );

    // Calculate status
    let status = (cpu.flags.n << 7) |
                 (cpu.flags.v << 6) |
                 (1 << 5) |          // B flag (set when PHP pushes)
                 (1 << 4) |          // Unused bit, always 1
                 (cpu.flags.d << 3) |
                 (cpu.flags.i << 2) |
                 (cpu.flags.z << 1) |
                 cpu.flags.c;

    println!(
        "Calculated status: {:#04x} (binary: {:08b})",
        status, status
    );

    println!("Expected test value: 0xB2 (binary: {:08b})", 0xB2);

    // Push status register to stack
    cpu.memory.data[0x0100 + cpu.registers.sp as usize] = status;
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);

    3 // Cycles
}
