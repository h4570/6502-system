use crate::cpu::cpu::Cpu;
use log::{log_enabled, trace};

/// Macro for logging 6502 processor state after instruction execution.
///
/// Displays:
/// - Instruction name
/// - Addressing mode
/// - Register values (A, X, Y, SP)
/// - Processor Status (P)
/// - Individual flag states (N, V, B, D, I, Z, C)
#[macro_export]
macro_rules! trace_instruction {
    ($cpu:expr, $opcode:expr, $mode:expr) => {
            trace!(
                "{:?}[{}] {} | A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} | N:{} V:{} B:{} D:{} I:{} Z:{} C:{}",
                stringify!($opcode),
                $mode,
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

/// Helper function to format operand bytes based on addressing mode
pub fn format_operand_bytes(cpu: &Cpu, bytes: u8) -> String {
    let pc = cpu.registers.pc as usize;
    match bytes {
        1 => format!("{:02X}", cpu.memory.data[pc]),
        2 => format!(
            "{:02X} {:02X}",
            cpu.memory.data[pc],
            cpu.memory.data[pc + 1]
        ),
        3 => format!(
            "{:02X} {:02X} {:02X}",
            cpu.memory.data[pc],
            cpu.memory.data[pc + 1],
            cpu.memory.data[pc + 2]
        ),
        _ => String::from(""),
    }
}
