use super::instructions::{
    adc, and, asl, bcc, bcs, beq, bit, bmi, bne, bpl, brk, bvc, bvs, clc, cld, cli, clv, cmp, cpx,
    cpy, dec, dex, dey, eor, inc, inx, iny, jmp, jsr, lda, ldx, ldy, lsr, nop, ora, pha, php, pla,
    plp, rol, ror, rti, rts, sbc, sec, sed, sei, sta, stx, sty, tax, tay, tsx, txa, txs, tya,
};
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
    // PHP
    cpu_instructions[0x08] = php::php_0x08;
    // PLP
    cpu_instructions[0x28] = plp::plp_0x28;
    // RTI
    cpu_instructions[0x40] = rti::rti_0x40;
    // BIT
    cpu_instructions[0x24] = bit::bit_0x24;
    cpu_instructions[0x2c] = bit::bit_0x2c;
    // ASL
    cpu_instructions[0x0A] = asl::asl_0x0a;
    cpu_instructions[0x06] = asl::asl_0x06;
    cpu_instructions[0x16] = asl::asl_0x16;
    cpu_instructions[0x0E] = asl::asl_0x0e;
    cpu_instructions[0x1E] = asl::asl_0x1e;
    // LSR
    cpu_instructions[0x4A] = lsr::lsr_0x4a;
    cpu_instructions[0x46] = lsr::lsr_0x46;
    cpu_instructions[0x56] = lsr::lsr_0x56;
    cpu_instructions[0x4E] = lsr::lsr_0x4e;
    cpu_instructions[0x5E] = lsr::lsr_0x5e;
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
    // AND
    cpu_instructions[0x29] = and::and_0x29;
    cpu_instructions[0x25] = and::and_0x25;
    cpu_instructions[0x35] = and::and_0x35;
    cpu_instructions[0x2D] = and::and_0x2d;
    cpu_instructions[0x3D] = and::and_0x3d;
    cpu_instructions[0x39] = and::and_0x39;
    cpu_instructions[0x21] = and::and_0x21;
    cpu_instructions[0x31] = and::and_0x31;
    // SBC
    cpu_instructions[0xE9] = sbc::sbc_0xe9;
    cpu_instructions[0xE5] = sbc::sbc_0xe5;
    cpu_instructions[0xF5] = sbc::sbc_0xf5;
    cpu_instructions[0xED] = sbc::sbc_0xed;
    cpu_instructions[0xFD] = sbc::sbc_0xfd;
    cpu_instructions[0xF9] = sbc::sbc_0xf9;
    cpu_instructions[0xE1] = sbc::sbc_0xe1;
    cpu_instructions[0xF1] = sbc::sbc_0xf1;
    // BRK
    cpu_instructions[0x00] = brk::brk_0x00;
    // ORA
    cpu_instructions[0x09] = ora::ora_0x09;
    cpu_instructions[0x05] = ora::ora_0x05;
    cpu_instructions[0x15] = ora::ora_0x15;
    cpu_instructions[0x0D] = ora::ora_0x0d;
    cpu_instructions[0x1D] = ora::ora_0x1d;
    cpu_instructions[0x19] = ora::ora_0x19;
    cpu_instructions[0x01] = ora::ora_0x01;
    cpu_instructions[0x11] = ora::ora_0x11;
    // EOR
    cpu_instructions[0x49] = eor::eor_0x49;
    cpu_instructions[0x45] = eor::eor_0x45;
    cpu_instructions[0x55] = eor::eor_0x55;
    cpu_instructions[0x4D] = eor::eor_0x4d;
    cpu_instructions[0x5D] = eor::eor_0x5d;
    cpu_instructions[0x59] = eor::eor_0x59;
    cpu_instructions[0x41] = eor::eor_0x41;
    cpu_instructions[0x51] = eor::eor_0x51;
    // INC
    cpu_instructions[0xE6] = inc::inc_0xe6;
    cpu_instructions[0xF6] = inc::inc_0xf6;
    cpu_instructions[0xEE] = inc::inc_0xee;
    cpu_instructions[0xFE] = inc::inc_0xfe;
    // INX
    cpu_instructions[0xE8] = inx::inx_0xe8;
    // INY
    cpu_instructions[0xC8] = iny::iny_0xc8;
    // DEC
    cpu_instructions[0xC6] = dec::dec_0xc6;
    cpu_instructions[0xD6] = dec::dec_0xd6;
    cpu_instructions[0xCE] = dec::dec_0xce;
    cpu_instructions[0xDE] = dec::dec_0xde;
    // DEX
    cpu_instructions[0xCA] = dex::dex_0xca;
    // DEY
    cpu_instructions[0x88] = dey::dey_0x88;
    cpu_instructions[0x4C] = jmp::jmp_0x4c;
    cpu_instructions[0x6C] = jmp::jmp_0x6c;
    // JSR
    cpu_instructions[0x20] = jsr::jsr_0x20;
    cpu_instructions[0x48] = pha::pha_0x48;
    // PLA
    cpu_instructions[0x68] = pla::pla_0x68;
    // RTS
    cpu_instructions[0x60] = rts::rts_0x60;
    // NOP
    cpu_instructions[0xEA] = nop::nop_0xea;
    // CMP
    cpu_instructions[0xC9] = cmp::cmp_0xc9;
    cpu_instructions[0xC5] = cmp::cmp_0xc5;
    cpu_instructions[0xD5] = cmp::cmp_0xd5;
    cpu_instructions[0xCD] = cmp::cmp_0xcd;
    cpu_instructions[0xDD] = cmp::cmp_0xdd;
    cpu_instructions[0xD9] = cmp::cmp_0xd9;
    cpu_instructions[0xC1] = cmp::cmp_0xc1;
    cpu_instructions[0xD1] = cmp::cmp_0xd1;
    // CPX
    cpu_instructions[0xE0] = cpx::cpx_0xe0;
    cpu_instructions[0xE4] = cpx::cpx_0xe4;
    cpu_instructions[0xEC] = cpx::cpx_0xec;
    // CPY
    cpu_instructions[0xC0] = cpy::cpy_0xc0;
    cpu_instructions[0xC4] = cpy::cpy_0xc4;
    cpu_instructions[0xCC] = cpy::cpy_0xcc;
    // Branch instructions
    cpu_instructions[0xF0] = beq::beq_0xf0; // BEQ
    cpu_instructions[0xD0] = bne::bne_0xd0; // BNE
    cpu_instructions[0x90] = bcc::bcc_0x90; // BCC
    cpu_instructions[0xB0] = bcs::bcs_0xb0; // BCS
    cpu_instructions[0x10] = bpl::bpl_0x10; // BPL
    cpu_instructions[0x30] = bmi::bmi_0x30; // BMI
    cpu_instructions[0x50] = bvc::bvc_0x50; // BVC
    cpu_instructions[0x70] = bvs::bvs_0x70; // BVS

    // Clear flag instructions
    cpu_instructions[0x18] = clc::clc_0x18; // CLC
    cpu_instructions[0xD8] = cld::cld_0xd8; // CLD
    cpu_instructions[0x58] = cli::cli_0x58; // CLI
    cpu_instructions[0xB8] = clv::clv_0xb8; // CLV

    // Set flag instructions
    cpu_instructions[0x38] = sec::sec_0x38; // SEC
    cpu_instructions[0xF8] = sed::sed_0xf8; // SED
    cpu_instructions[0x78] = sei::sei_0x78; // SEI

    // ROL
    cpu_instructions[0x2A] = rol::rol_0x2a; // ROL A
    cpu_instructions[0x26] = rol::rol_0x26; // ROL Zero Page
    cpu_instructions[0x36] = rol::rol_0x36; // ROL Zero Page,X
    cpu_instructions[0x2E] = rol::rol_0x2e; // ROL Absolute
    cpu_instructions[0x3E] = rol::rol_0x3e; // ROL Absolute,X

    // ROR
    cpu_instructions[0x6A] = ror::ror_0x6a; // ROR A
    cpu_instructions[0x66] = ror::ror_0x66; // ROR Zero Page
    cpu_instructions[0x76] = ror::ror_0x76; // ROR Zero Page,X
    cpu_instructions[0x6E] = ror::ror_0x6e; // ROR Absolute
    cpu_instructions[0x7E] = ror::ror_0x7e; // ROR Absolute,X
}
