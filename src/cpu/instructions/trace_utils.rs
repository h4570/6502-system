use crate::cpu::cpu::Cpu;

/// Returns processor status byte combining all flags
pub(crate) fn get_processor_status(cpu: &Cpu) -> u8 {
    (cpu.flags.n << 7)     // Negative flag (bit 7)
            | (cpu.flags.v << 6)    // Overflow flag (bit 6)
            | (1 << 5)               // Unused bit, always set to 1 (bit 5)
            | (cpu.flags.b << 4)    // Break flag (bit 4)
            | (cpu.flags.d << 3)    // Decimal mode flag (bit 3)
            | (cpu.flags.i << 2)    // Interrupt disable flag (bit 2)
            | (cpu.flags.z << 1)    // Zero flag (bit 1)
            | cpu.flags.c // Carry flag (bit 0)
}

/// Macro for logging 6502 processor state after instruction execution.
///
/// Displays:
/// - Instruction name
/// - Opcode
/// - Addressing mode
/// - Register values (A, X, Y, SP)
/// - Processor Status (P)
/// - Individual flag states (N, V, B, D, I, Z, C)
#[macro_export]
macro_rules! trace_instruction {
    ($cpu:expr, $instr:expr, $opcode:expr, $mode:expr) => {
            log::trace!(
                "{}[{}] ({}) | A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} | N:{} V:{} B:{} D:{} I:{} Z:{} C:{}",
                $instr,
                $opcode,
                $mode,
                $cpu.registers.a,
                $cpu.registers.x,
                $cpu.registers.y,
                crate::cpu::instructions::trace_utils::get_processor_status($cpu),
                $cpu.registers.sp,
                $cpu.flags.n,
                $cpu.flags.v,
                $cpu.flags.b,
                $cpu.flags.d,
                $cpu.flags.i,
                $cpu.flags.z,
                $cpu.flags.c
            );
    };
}
