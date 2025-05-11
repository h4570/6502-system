use crate::cpu::cpu::Cpu;

/// Immediate addressing mode
///
/// Returns the next byte from memory as an immediate operand value rather than an address.
/// This is used when an instruction operates on a constant value.
///
/// Cycles: 2 (1 for opcode, 1 for immediate value)
pub(crate) fn addr_immediate(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_byte();
    val
}

/// Zero Page addressing mode
///
/// Fetches a single byte which is used as an address in the zero page ($0000-$00FF).
/// This is a faster addressing mode as it only requires a single byte for the address.
///
/// Cycles: 2 (1 for opcode, 1 for zero page address)
pub(crate) fn addr_zeropage(cpu: &mut Cpu) -> u16 {
    let addr = cpu.fetch_byte() as u16;
    addr
}

/// Zero Page X-Indexed addressing mode
///
/// Fetches a single byte and adds the X register to it (with wrap-around in the zero page).
/// The resulting value is used as an address in the zero page ($0000-$00FF).
///
/// Cycles: 3 (1 for opcode, 1 for zero page address, 1 for indexing)
pub(crate) fn addr_zeropage_x(cpu: &mut Cpu) -> u16 {
    let addr = (cpu.fetch_byte() + cpu.registers.x) & 0xFF;
    addr as u16
}

/// Zero Page Y-Indexed addressing mode
///
/// Fetches a single byte and adds the Y register to it (with wrap-around in the zero page).
/// The resulting value is used as an address in the zero page ($0000-$00FF).
///
/// Cycles: 3 (1 for opcode, 1 for zero page address, 1 for indexing)
pub(crate) fn addr_zeropage_y(cpu: &mut Cpu) -> u16 {
    let addr = (cpu.fetch_byte() + cpu.registers.y) & 0xFF;
    addr as u16
}

/// Absolute addressing mode
///
/// Fetches a full 16-bit address (low byte first, then high byte) to access any location in memory.
///
/// Cycles: 3 (1 for opcode, 2 for the 16-bit address)
pub(crate) fn addr_absolute(cpu: &mut Cpu) -> u16 {
    let addr_lo: u16 = cpu.fetch_byte() as u16;
    let addr_hi = cpu.fetch_byte() as u16;
    let addr = (addr_hi << 8) | addr_lo;
    addr
}

/// Absolute X-Indexed addressing mode
///
/// Fetches a full 16-bit address and adds the X register to it.
/// Returns both the computed address and whether a page boundary was crossed.
/// Page crossing may add an additional cycle to instruction execution.
///
/// Cycles: 3 without page crossing, 4 with page crossing
/// (1 for opcode, 2 for the 16-bit address, 0-1 for page crossing)
pub(crate) fn addr_absolute_x(cpu: &mut Cpu) -> (u16, bool) {
    let addr_lo = cpu.fetch_byte() as u16;
    let addr_hi = cpu.fetch_byte() as u16;
    let base = (addr_hi << 8) | addr_lo;
    let addr = base + (cpu.registers.x as u16);
    let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
    (addr, page_crossed)
}

/// Absolute Y-Indexed addressing mode
///
/// Fetches a full 16-bit address and adds the Y register to it.
/// Returns both the computed address and whether a page boundary was crossed.
/// Page crossing may add an additional cycle to instruction execution.
///
/// Cycles: 3 without page crossing, 4 with page crossing
/// (1 for opcode, 2 for the 16-bit address, 0-1 for page crossing)
pub(crate) fn addr_absolute_y(cpu: &mut Cpu) -> (u16, bool) {
    let addr_lo = cpu.fetch_byte() as u16;
    let addr_hi = cpu.fetch_byte() as u16;
    let base = (addr_hi << 8) | addr_lo;
    let addr = base + (cpu.registers.y as u16);
    let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
    (addr, page_crossed)
}

/// Indexed Indirect (X) addressing mode
///
/// Often called "(Indirect,X)". Fetches a zero page address, adds X to it (with wrap-around),
/// then reads a 16-bit address from that location in the zero page (and the next byte).
/// Useful for selecting from a table of addresses.
///
/// Cycles: 5 (1 for opcode, 1 for zero page address, 1 for indexing, 2 for indirection)
pub(crate) fn addr_indirect_x(cpu: &mut Cpu) -> u16 {
    let operand = cpu.fetch_byte();
    let zp_addr = (operand + cpu.registers.x) & 0xFF;

    let addr_lo = cpu.memory.data[zp_addr as usize] as u16;
    let addr_hi = cpu.memory.data[(zp_addr as u8).wrapping_add(1) as usize] as u16;
    let addr = (addr_hi << 8) | addr_lo;

    addr
}

/// Indirect Indexed (Y) addressing mode
///
/// Often called "(Indirect),Y". Fetches a zero page address, reads a 16-bit address from that
/// location in the zero page (and the next byte), then adds Y to that address.
/// Returns both the computed address and whether a page boundary was crossed.
///
/// Cycles: 4 without page crossing, 5 with page crossing
/// (1 for opcode, 1 for zero page address, 2 for indirection, 0-1 for page crossing)
pub(crate) fn addr_indirect_y(cpu: &mut Cpu) -> (u16, bool) {
    let zp_addr = cpu.fetch_byte();

    let addr_lo = cpu.memory.data[zp_addr as usize] as u16;
    let addr_hi = cpu.memory.data[(zp_addr as u8).wrapping_add(1) as usize] as u16;
    let base = (addr_hi << 8) | addr_lo;
    let addr = base + (cpu.registers.y as u16);

    let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
    (addr, page_crossed)
}
