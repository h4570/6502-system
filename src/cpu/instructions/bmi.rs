use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bmi_0x30(cpu: &mut Cpu) -> u8 {
    // Branch if negative flag is set (N=1)
    let cycles = branch_if(cpu, cpu.flags.n == 1, "BMI");
    trace_instruction!(cpu, "BMI", "0x30", "Relative");
    cycles
}
