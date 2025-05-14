use crate::cpu::cpu::Cpu;

/// Prints current CPU state for debugging
pub(crate) fn trace_state(cpu: &mut Cpu) -> String {
    format!(
        "A:{:02X} X:{:02X} Y:{:02X} PC:{:04X} SP:{:02X} | N:{} V:{} B:{} D:{} I:{} Z:{} C:{}",
        cpu.registers.a,
        cpu.registers.x,
        cpu.registers.y,
        cpu.registers.pc,
        cpu.registers.sp,
        cpu.flags.n,
        cpu.flags.v,
        cpu.flags.b,
        cpu.flags.d,
        cpu.flags.i,
        cpu.flags.z,
        cpu.flags.c
    )
}
