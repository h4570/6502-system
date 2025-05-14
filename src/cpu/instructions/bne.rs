use crate::cpu::cpu::Cpu;
use crate::cpu::instructions::branch_utils::branch_if;
use crate::trace_instruction;

pub(crate) fn bne_0xd0(cpu: &mut Cpu) -> u8 {
    // Branch if zero flag is clear (Z=0)
    let cycles = branch_if(cpu, cpu.flags.z == 0, "BNE");
    trace_instruction!(cpu, "BNE", "0xD0", "Relative");
    cycles
}
