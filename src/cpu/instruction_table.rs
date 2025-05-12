use super::instructions::{adc, brk, lda, ldx, ldy, sta, stx, sty, tax, tay, tsx, txa, txs, tya};
use crate::cpu::cpu::Cpu;

pub(crate) fn arrange_instruction_table(cpu_instructions: &mut [fn(&mut Cpu) -> u8; 256]) {
    // LDA
    cpu_instructions[0xA9] = lda::lda_0xa9;
    cpu_instructions[0xA5] = lda::lda_0xa5;
    cpu_instructions[0xB5] = lda::lda_0xb5;
    cpu_instructions[0xAD] = lda::lda_0xad;
    cpu_instructions[0xBD] = lda::lda_0xbd;
    cpu_instructions[0xB9] = lda::lda_0xb9;
    cpu_instructions[0xA1] = lda::lda_0xa1;
    cpu_instructions[0xB1] = lda::lda_0xb1;
    // LDX
    cpu_instructions[0xA2] = ldx::ldx_0xa2;
    cpu_instructions[0xA6] = ldx::ldx_0xa6;
    cpu_instructions[0xB6] = ldx::ldx_0xb6;
    cpu_instructions[0xAE] = ldx::ldx_0xae;
    cpu_instructions[0xBE] = ldx::ldx_0xbe;
    // LDY
    cpu_instructions[0xA0] = ldy::ldy_0xa0;
    cpu_instructions[0xA4] = ldy::ldy_0xa4;
    cpu_instructions[0xB4] = ldy::ldy_0xb4;
    cpu_instructions[0xAC] = ldy::ldy_0xac;
    cpu_instructions[0xBC] = ldy::ldy_0xbc;
    // STA
    cpu_instructions[0x85] = sta::sta_0x85;
    cpu_instructions[0x95] = sta::sta_0x95;
    cpu_instructions[0x8D] = sta::sta_0x8d;
    cpu_instructions[0x9D] = sta::sta_0x9d;
    cpu_instructions[0x99] = sta::sta_0x99;
    cpu_instructions[0x81] = sta::sta_0x81;
    cpu_instructions[0x91] = sta::sta_0x91;
    // STX
    cpu_instructions[0x86] = stx::stx_0x86;
    cpu_instructions[0x96] = stx::stx_0x96;
    cpu_instructions[0x8E] = stx::stx_0x8e;
    // STY
    cpu_instructions[0x84] = sty::sty_0x84;
    cpu_instructions[0x94] = sty::sty_0x94;
    cpu_instructions[0x8C] = sty::sty_0x8c;
    // TAX
    cpu_instructions[0xAA] = tax::tax_0xaa;
    // TAY
    cpu_instructions[0xA8] = tay::tay_0xa8;
    // TSX
    cpu_instructions[0xBA] = tsx::tsx_0xba;
    // TXA
    cpu_instructions[0x8A] = txa::txa_0x8a;
    // TXS
    cpu_instructions[0x9A] = txs::txs_0x9a;
    // TYA
    cpu_instructions[0x98] = tya::tya_0x98;
    // ADC
    cpu_instructions[0x69] = adc::adc_0x69;
    cpu_instructions[0x65] = adc::adc_0x65;
    cpu_instructions[0x75] = adc::adc_0x75;
    cpu_instructions[0x6D] = adc::adc_0x6d;
    cpu_instructions[0x7D] = adc::adc_0x7d;
    cpu_instructions[0x79] = adc::adc_0x79;
    cpu_instructions[0x61] = adc::adc_0x61;
    cpu_instructions[0x71] = adc::adc_0x71;
    // BRK
    cpu_instructions[0x00] = brk::brk_0x00;
}
