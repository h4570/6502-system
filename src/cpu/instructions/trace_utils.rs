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
                $cpu.get_processor_status(),
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
