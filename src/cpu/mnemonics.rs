use super::alu;
use super::addressing::Addressing;
use super::addressing::MemoryCell;
use super::addressing::stack_push;
use super::addressing::stack_pull;
use super::register::Register;

#[derive(Copy, Clone, Debug)]
pub enum Mnemonics {
    NUL,
    ADC(Addressing), AND(Addressing), ASL(Addressing), BCC(Addressing), BCS(Addressing), BEQ(Addressing),
    BIT(Addressing), BMI(Addressing), BNE(Addressing), BPL(Addressing), BRK(Addressing), BVC(Addressing),
    BVS(Addressing), CLC(Addressing), CLD(Addressing), CLI(Addressing), CLV(Addressing), CMP(Addressing),
    CPX(Addressing), CPY(Addressing), DEC(Addressing), DEX(Addressing), DEY(Addressing), EOR(Addressing),
    INC(Addressing), INX(Addressing), INY(Addressing), JMP(Addressing), JSR(Addressing), LDA(Addressing),
    LDX(Addressing), LDY(Addressing), LSR(Addressing), NOP(Addressing), ORA(Addressing), PHA(Addressing),
    PHP(Addressing), PLA(Addressing), PLP(Addressing), ROL(Addressing), ROR(Addressing), RTI(Addressing),
    RTS(Addressing), SBC(Addressing), SEC(Addressing), SED(Addressing), SEI(Addressing), STA(Addressing),
    STX(Addressing), STY(Addressing), TAX(Addressing), TAY(Addressing), TSX(Addressing), TXA(Addressing),
    TXS(Addressing), TYA(Addressing)
}

impl Mnemonics {
    pub fn handle(&self, register: &mut Register, memory: &mut [u8]) -> u8 {
        match self {
            Mnemonics::ADC(addressing) => {
                let cell = addressing.read(memory, register);
                adc(cell, register)
            },
            Mnemonics::AND(addressing) => {
                let cell = addressing.read(memory, register);
                and(cell, register)
            },
            Mnemonics::ASL(addressing) => {
                let cell = addressing.read(memory, register);
                asl(memory, cell, register)
            },
            Mnemonics::BCC(addressing) => {
                let cell = addressing.read(memory, register);
                bcc(cell, register)
            },
            Mnemonics::BCS(addressing) => {
                let cell = addressing.read(memory, register);
                bcs(cell, register)
            },
            Mnemonics::BEQ(addressing) => {
                let cell = addressing.read(memory, register);
                beq(cell, register)
            },
            Mnemonics::BIT(addressing) => {
                let cell = addressing.read(memory, register);
                bit(cell, register)
            },
            Mnemonics::BMI(addressing) => {
                let cell = addressing.read(memory, register);
                bmi(cell, register)
            },
            Mnemonics::BNE(addressing) => {
                let cell = addressing.read(memory, register);
                bne(cell, register)
            },
            Mnemonics::BPL(addressing) => {
                let cell = addressing.read(memory, register);
                bpl(cell, register)
            },
            Mnemonics::BRK(_addressing) => { brk(memory, register) },
            Mnemonics::BVC(addressing) => {
                let cell = addressing.read(memory, register);
                bvc(cell, register)
            },
            Mnemonics::BVS(addressing) => {
                let cell = addressing.read(memory, register);
                bvs(cell, register)
            },
            Mnemonics::CLC(_addressing) => { clc(register) },
            Mnemonics::CLD(_addressing) => { cld(register) },
            Mnemonics::CLI(_addressing) => { cli(register) },
            Mnemonics::CLV(_addressing) => { clv(register) },
            Mnemonics::CMP(addressing) => {
                let cell = addressing.read(memory, register);
                cmp(cell, register)
            },
            Mnemonics::CPX(addressing) => {
                let cell = addressing.read(memory, register);
                cpx(cell, register)
            },
            Mnemonics::CPY(addressing) => {
                let cell = addressing.read(memory, register);
                cpy(cell, register)
            },
            Mnemonics::DEC(addressing) => {
                let cell = addressing.read(memory, register);
                dec(memory, cell, register)
            },
            Mnemonics::DEX(_addressing) => { dex(register) },
            Mnemonics::DEY(_addressing) => { dey(register) },
            Mnemonics::EOR(addressing) => {
                let cell = addressing.read(memory, register);
                eor(cell, register)
            },
            Mnemonics::INC(addressing) => {
                let cell = addressing.read(memory, register);
                inc(memory, cell, register)
            },
            Mnemonics::INX(_addressing) => { inx(register) },
            Mnemonics::INY(_addressing) => { iny(register) },
            Mnemonics::JMP(addressing) => {
                let cell = addressing.read(memory, register);
                jmp(cell, register)
            },
            Mnemonics::JSR(addressing) => {
                let cell = addressing.read(memory, register);
                jsr(memory, cell, register)
            },
            Mnemonics::LDA(addressing) => {
                let cell = addressing.read(memory, register);
                lda(cell, register)
            },
            Mnemonics::LDX(addressing) => {
                let cell = addressing.read(memory, register);
                ldx(cell, register)
            },
            Mnemonics::LDY(addressing) => {
                let cell = addressing.read(memory, register);
                ldy(cell, register)
            },
            Mnemonics::LSR(addressing) => {
                let cell = addressing.read(memory, register);
                lsr(memory, cell, register)
            },
            Mnemonics::NOP(_addressing) => { nop() },
            Mnemonics::ORA(addressing) => {
                let cell = addressing.read(memory, register);
                ora(cell, register)
            },
            Mnemonics::PHA(_addressing) => { pha(memory, register) },
            Mnemonics::PHP(_addressing) => { php(memory, register) },
            Mnemonics::PLA(_addressing) => { pla(memory, register) },
            Mnemonics::PLP(_addressing) => { plp(memory, register) },
            Mnemonics::ROL(addressing) => {
                let cell = addressing.read(memory, register);
                rol(memory, cell, register)
            },
            Mnemonics::ROR(addressing) => {
                let cell = addressing.read(memory, register);
                ror(memory, cell, register)
            },
            Mnemonics::RTI(_addressing) => { rti(memory, register) },
            Mnemonics::RTS(_addressing) => { rts(memory, register) },
            Mnemonics::SBC(addressing) => {
                let cell = addressing.read(memory, register);
                sbc(cell, register)
            },
            Mnemonics::SEC(_addressing) => { sec(register) },
            Mnemonics::SED(_addressing) => { sed(register) },
            Mnemonics::SEI(_addressing) => { sei(register) },
            Mnemonics::STA(addressing) => {
                let cell = addressing.read(memory, register);
                sta(memory, cell, register)
            },
            Mnemonics::STX(addressing) => {
                let cell = addressing.read(memory, register);
                stx(memory, cell, register)
            },
            Mnemonics::STY(addressing) => {
                let cell = addressing.read(memory, register);
                sty(memory, cell, register)
            },
            Mnemonics::TAX(_addressing) => { tax(register) },
            Mnemonics::TAY(_addressing) => { tay(register) },
            Mnemonics::TSX(_addressing) => { tsx(register) },
            Mnemonics::TXA(_addressing) => { txa(register) },
            Mnemonics::TXS(_addressing) => { txs(register) },
            Mnemonics::TYA(_addressing) => { tya(register) },
            Mnemonics::NUL => panic!("NULL")
        }
    }
}

fn adc(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::add(register.a, cell.value, register.carry_bit(), register.decimal_bit());

    register.a = result.value;
    set_nvzc_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn and(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::and(register.a, cell.value);

    register.a = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn asl(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::shift_left(cell.value);
    let result_value = result.value;
    set_nzc_from_alu_result_bits(register, result);

    if cell.cycles == 0 {
        register.a = result_value;
        return 2;
    }

    memory[cell.address] = result_value;
    return 4 + cell.cycles
}

fn bcc(cell: MemoryCell, register: &mut Register) -> u8 {
    if register.carry_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn bcs(cell: MemoryCell, register: &mut Register) -> u8 {
    if !register.carry_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn beq(cell: MemoryCell, register: &mut Register) -> u8 {
    if !register.zero_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn bit(cell: MemoryCell, register: &mut Register) -> u8 {
    register.set_negative_bit(cell.value & 0x80 == 0x80);
    register.set_overflow_bit(cell.value & 0x40 == 0x40);
    register.set_zero_bit(cell.value & register.a == 0);

    return 2 + cell.cycles;
}

fn bmi(cell: MemoryCell, register: &mut Register) -> u8 {
    if !register.negative_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn bne(cell: MemoryCell, register: &mut Register) -> u8 {
    if register.zero_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn bpl(cell: MemoryCell, register: &mut Register) -> u8 {
    if register.negative_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn brk(memory: &mut [u8], register: &mut Register) -> u8 {
    register.increment_pc();
    stack_push(memory, register, (register.pc() >> 8) as u8);
    stack_push(memory, register, register.pc() as u8);

    register.set_break_bit(true);
    stack_push(memory, register, register.p());

    let pc_low = memory[0xfffe];
    let pc_high = memory[0xffff];

    register.set_pc(((pc_high as u16) << 8) + pc_low as u16);
    register.set_interrupt_bit(true);

    return 7;
}

fn bvc(cell: MemoryCell, register: &mut Register) -> u8 {
    if register.overflow_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn bvs(cell: MemoryCell, register: &mut Register) -> u8 {
    if !register.overflow_bit() { return 2; }

    register.set_pc(cell.address as u16);
    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn clc(register: &mut Register) -> u8 {
    register.set_carry_bit(false);

    return 2;
}

fn cld(register: &mut Register) -> u8 {
    register.set_decimal_bit(false);

    return 2;
}

fn cli(register: &mut Register) -> u8 {
    register.set_interrupt_bit(false);

    return 2;
}

fn clv(register: &mut Register) -> u8 {
    register.set_overflow_bit(false);

    return 2;
}

fn cmp(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::subtract(register.a, cell.value, true, false);
    set_nzc_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn cpx(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::subtract(register.x, cell.value, true, false);
    set_nzc_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn cpy(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::subtract(register.y, cell.value, true, false);
    set_nzc_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn dec(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::decrement(cell.value);
    memory[cell.address] = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 4 + cell.cycles;
}

fn dex(register: &mut Register) -> u8 {
    let result = alu::decrement(register.x);
    register.x = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2;
}

fn dey(register: &mut Register) -> u8 {
    let result = alu::decrement(register.y);
    register.y = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2;
}

fn eor(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::xor(register.a, cell.value);

    register.a = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn inc(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::increment(cell.value);
    memory[cell.address] = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 4 + cell.cycles;
}

fn inx(register: &mut Register) -> u8 {
    let result = alu::increment(register.x);
    register.x = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2;
}

fn iny(register: &mut Register) -> u8 {
    let result = alu::increment(register.y);
    register.y = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2;
}

fn jmp(cell: MemoryCell, register: &mut Register) -> u8 {
    register.set_pc(cell.address as u16);
    return 1 + cell.cycles;
}

fn jsr(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    register.set_pc(register.pc() - 1);
    stack_push(memory, register, ((register.pc() & 0xff00) >> 8) as u8);
    stack_push(memory, register, register.pc() as u8);
    register.set_pc(cell.address as u16);

    return 6;
}

fn lda(cell: MemoryCell, register: &mut Register) -> u8 {
    register.a = cell.value;
    set_nz_from_raw_result_bits(register, cell.value);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn ldx(cell: MemoryCell, register: &mut Register) -> u8 {
    register.x = cell.value;
    set_nz_from_raw_result_bits(register, cell.value);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn ldy(cell: MemoryCell, register: &mut Register) -> u8 {
    register.y = cell.value;
    set_nz_from_raw_result_bits(register, cell.value);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn lsr(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::shift_right(cell.value);
    let result_value = result.value;
    set_nzc_from_alu_result_bits(register, result);

    if cell.cycles == 0 {
        register.a = result_value;
        return 2;
    }

    memory[cell.address] = result_value;
    return 4 + cell.cycles
}

fn nop() -> u8 {
    return 2;
}

fn ora(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::or(register.a, cell.value);

    register.a = result.value;
    set_nz_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn pha(memory: &mut [u8], register: &mut Register) -> u8 {
    stack_push(memory, register, register.a);

    return 3;
}

fn php(memory: &mut [u8], register: &mut Register) -> u8 {
    stack_push(memory, register, register.p() | 0x30);

    return 3;
}

fn pla(memory: &[u8], register: &mut Register) -> u8 {
    register.a = stack_pull(memory, register);
    set_nz_from_raw_result_bits(register, register.a);

    return 4;
}

fn plp(memory: &[u8], register: &mut Register) -> u8 {
    let status_register = stack_pull(memory, register);
    register.set_p(status_register);

    return 4;
}

fn rol(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let previous_carry_bit = register.carry_bit();
    let result = alu::shift_left(cell.value);
    let result_value = if previous_carry_bit { result.value | 0x01 } else { result.value & 0xFE };
    set_nz_from_raw_result_bits(register, result_value);
    register.set_carry_bit(result.carry);

    if cell.cycles == 0 {
        register.a = result_value;
        return 2;
    }

    memory[cell.address] = result_value;
    return 4 + cell.cycles
}

fn ror(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    let previous_carry_bit = register.carry_bit();
    let result = alu::shift_right(cell.value);
    let result_value = if previous_carry_bit { result.value | 0x80 } else { result.value & 0x7f };
    set_nz_from_raw_result_bits(register, result_value);
    register.set_carry_bit(result.carry);

    if cell.cycles == 0 {
        register.a = result_value;
        return 2;
    }

    memory[cell.address] = result_value;
    return 4 + cell.cycles
}

fn rti(memory: &[u8], register: &mut Register) -> u8 {
    let status_register = stack_pull(memory, register);
    let pc_low = stack_pull(memory, register);
    let pc_high = stack_pull(memory, register);

    register.set_p(status_register);
    register.set_break_bit(false);
    register.set_pc(((pc_high as u16) << 8) + pc_low as u16);

    return 6;
}

fn rts(memory: &[u8], register: &mut Register) -> u8 {
    let pc_low = stack_pull(memory, register);
    let pc_high = stack_pull(memory, register);

    register.set_pc((((pc_high as u16) << 8) + pc_low as u16).overflowing_add(1).0);

    return 6;
}

fn sbc(cell: MemoryCell, register: &mut Register) -> u8 {
    let result = alu::subtract(register.a, cell.value, register.carry_bit(), register.decimal_bit());

    register.a = result.value;
    set_nvzc_from_alu_result_bits(register, result);

    return 2 + cell.cycles + if cell.in_bounds { 0 } else { 1 };
}

fn sec(register: &mut Register) -> u8 {
    register.set_carry_bit(true);

    return 2;
}

fn sed(register: &mut Register) -> u8 {
    register.set_decimal_bit(true);

    return 2;
}

fn sei(register: &mut Register) -> u8 {
    register.set_interrupt_bit(true);

    return 2;
}

fn sta(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    memory[cell.address] = register.a;

    return 2 + cell.cycles;
}

fn stx(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    memory[cell.address] = register.x;

    return 2 + cell.cycles;
}

fn sty(memory: &mut [u8], cell: MemoryCell, register: &mut Register) -> u8 {
    memory[cell.address] = register.y;

    return 2 + cell.cycles;
}

fn tax(register: &mut Register) -> u8 {
    register.x = register.a;
    set_nz_from_raw_result_bits(register, register.a);

    return 2;
}

fn tay(register: &mut Register) -> u8 {
    register.y = register.a;
    set_nz_from_raw_result_bits(register, register.a);

    return 2;
}

fn tsx(register: &mut Register) -> u8 {
    register.x = register.s();
    set_nz_from_raw_result_bits(register, register.s());

    return 2;
}

fn txa(register: &mut Register) -> u8 {
    register.a = register.x;
    set_nz_from_raw_result_bits(register, register.x);

    return 2;
}

fn txs(register: &mut Register) -> u8 {
    register.set_s(register.x);

    return 2;
}

fn tya(register: &mut Register) -> u8 {
    register.a = register.y;
    set_nz_from_raw_result_bits(register, register.y);

    return 2;
}

// --------------------------------------------------------------------

fn set_nvzc_from_alu_result_bits(register: &mut Register, result: alu::AluResult) {
    register.set_overflow_bit(result.overflow);
    set_nzc_from_alu_result_bits(register, result);
}

fn set_nzc_from_alu_result_bits(register: &mut Register, result: alu::AluResult) {
    register.set_carry_bit(result.carry);
    set_nz_from_alu_result_bits(register, result);
}

fn set_nz_from_alu_result_bits(register: &mut Register, result: alu::AluResult) {
    register.set_negative_bit(result.negative);
    register.set_zero_bit(result.zero);
}

fn set_nz_from_raw_result_bits(register: &mut Register, result: u8) {
    register.set_negative_bit(result > 127);
    register.set_zero_bit(result == 0);
}

// TESTS


#[cfg(test)]
mod tests {
    use crate::cpu::alu;
    use crate::cpu::addressing::Addressing;
    use crate::cpu::addressing::MemoryCell;
    use crate::cpu::addressing::stack_push;
    use crate::cpu::addressing::stack_pull;
    use crate::cpu::register::Register;

    fn cell(value: u8, in_bounds: bool, cycles: u8) -> MemoryCell {
        MemoryCell {
            address: 0x02,
            value: value,
            in_bounds: in_bounds,
            cycles: cycles,
            bytes: 2
        }
    }

    #[test]
    fn test_adc() {
        use super::adc;

        let cell_in_bounds = cell(0x42, true, 5);
        let cell_out_of_bounds = cell(0x3B, false, 5);
        let mut register = Register::new();
        register.a = 0x03;

        let cycles = adc(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x45);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 7);

        let cycles = adc(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x80);
        assert_eq!(register.p(), 0b1110_0000);
        assert_eq!(cycles, 8);

        register.a = 0xBE;
        let cell_in_bounds = cell(0x42, true, 5);
        let cycles = adc(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_and() {
        use super::and;

        let cell_in_bounds = cell(0xA7, true, 5);
        let cell_out_of_bounds = cell(0x3C, false, 5);
        let mut register = Register::new();
        register.a = 0x91;

        let cycles = and(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x81);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        let cycles = and(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_asl() {
        use super::asl;

        let cell_immediate = cell(0xCC, true, 0);
        let cell_in_bounds = cell(0x80, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = asl(&mut memory, cell_immediate, &mut register);
        assert_eq!(register.a, 0x98);
        assert_eq!(register.p(), 0b1010_0001);
        assert_eq!(cycles, 2);

        let cycles = asl(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x00);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_bcc() {
        use super::bcc;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_carry_bit(false);
        let cycles = bcc(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bcc(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_carry_bit(true);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bcc(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_bcs() {
        use super::bcs;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_carry_bit(true);
        let cycles = bcs(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bcs(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_carry_bit(false);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bcs(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_beq() {
        use super::beq;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_zero_bit(true);
        let cycles = beq(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = beq(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_zero_bit(false);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = beq(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_bit() {
        use super::bit;

        let cell = cell(0xC0, true, 5);
        let mut register = Register::new();
        register.a = 0x3F;

        let cycles = bit(cell, &mut register);
        assert_eq!(register.a, 0x3F);
        assert_eq!(register.p(), 0b1110_0010);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_bmi() {
        use super::bmi;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_negative_bit(true);
        let cycles = bmi(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bmi(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_negative_bit(false);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bmi(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_bne() {
        use super::bne;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_zero_bit(false);
        let cycles = bne(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bne(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_zero_bit(true);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bne(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_bpl() {
        use super::bpl;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_negative_bit(false);
        let cycles = bpl(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bpl(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_negative_bit(true);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bpl(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_brk() {
        use super::brk;

        let mut memory: [u8; 65536] = [0; 65536];
        memory[0xfffe] = 0x20;
        memory[0xffff] = 0x04;
        let mut register = Register::new();
        register.set_pc(0x0305);

        let cycles = brk(&mut memory, &mut register);
        assert_eq!(register.pc(), 0x0420);
        assert_eq!(register.p(), 0b0011_0100);
        assert_eq!(register.s(), 0xFC);
        assert_eq!(memory[0x01ff], 0x03);
        assert_eq!(memory[0x01fe], 0x06);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_bvc() {
        use super::bvc;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_overflow_bit(false);
        let cycles = bvc(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bvc(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_overflow_bit(true);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bvc(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_bvs() {
        use super::bvs;

        let cell_in_bounds = cell(0xCC, true, 5);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let mut register = Register::new();
        register.set_pc(0x400);

        register.set_overflow_bit(true);
        let cycles = bvs(cell_in_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 7);

        let cycles = bvs(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 8);

        register.set_pc(0x400);
        register.set_overflow_bit(false);
        let cell_out_of_bounds = cell(0xCC, false, 5);
        let cycles = bvs(cell_out_of_bounds, &mut register);
        assert_eq!(register.pc(), 0x400);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_clc() {
        use super::clc;

        let mut register = Register::new();
        register.set_carry_bit(true);

        let cycles = clc(&mut register);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cld() {
        use super::cld;

        let mut register = Register::new();
        register.set_decimal_bit(true);

        let cycles = cld(&mut register);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cli() {
        use super::cli;

        let mut register = Register::new();
        register.set_interrupt_bit(true);

        let cycles = cli(&mut register);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_clv() {
        use super::clv;

        let mut register = Register::new();
        register.set_overflow_bit(true);

        let cycles = clv(&mut register);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cmp() {
        use super::cmp;

        let cell_in_bounds = cell(0x42, true, 5);
        let cell_out_of_bounds = cell(0x62, false, 5);
        let mut register = Register::new();
        register.a = 0x42;

        let cycles = cmp(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 7);

        let cycles = cmp(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 8);

        register.a = 0x42;
        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cmp(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 7);

        register.a = 0x42;
        register.set_decimal_bit(true);

        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cmp(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b0010_1000);
        assert_eq!(cycles, 7);

        register.a = 0x80;
        register.set_decimal_bit(false);

        let cell_in_bounds = cell(0x01, true, 5);
        let cycles = cmp(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x80);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_cpx() {
        use super::cpx;

        let cell_in_bounds = cell(0x42, true, 5);
        let cell_out_of_bounds = cell(0x62, false, 5);
        let mut register = Register::new();
        register.a = 0x99;
        register.x = 0x42;

        let cycles = cpx(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.x, 0x42);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 7);

        let cycles = cpx(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 8);

        register.a = 0x99;
        register.x = 0x42;
        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cpx(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.x, 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 7);

        register.a = 0x99;
        register.x = 0x42;
        register.set_decimal_bit(true);

        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cpx(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.x, 0x42);
        assert_eq!(register.p(), 0b0010_1000);
        assert_eq!(cycles, 7);

        register.a = 0x99;
        register.x = 0x80;
        register.set_decimal_bit(false);

        let cell_in_bounds = cell(0x01, true, 5);
        let cycles = cpx(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.x, 0x80);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_cpy() {
        use super::cpy;

        let cell_in_bounds = cell(0x42, true, 5);
        let cell_out_of_bounds = cell(0x62, false, 5);
        let mut register = Register::new();
        register.a = 0x99;
        register.y = 0x42;

        let cycles = cpy(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.y, 0x42);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 7);

        let cycles = cpy(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 8);

        register.a = 0x99;
        register.y = 0x42;
        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cpy(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.y, 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 7);

        register.a = 0x99;
        register.y = 0x42;
        register.set_decimal_bit(true);

        let cell_in_bounds = cell(0xF2, true, 5);
        let cycles = cpy(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.y, 0x42);
        assert_eq!(register.p(), 0b0010_1000);
        assert_eq!(cycles, 7);

        register.a = 0x99;
        register.y = 0x80;
        register.set_decimal_bit(false);

        let cell_in_bounds = cell(0x01, true, 5);
        let cycles = cpy(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x99);
        assert_eq!(register.y, 0x80);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_dec() {
        use super::dec;

        let cell = cell(0xCC, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = dec(&mut memory, cell, &mut register);
        assert_eq!(memory[0x02], 0xCB);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_dex() {
        use super::dex;

        let mut register = Register::new();
        register.x = 0xCC;

        let cycles = dex(&mut register);
        assert_eq!(register.x, 0xCB);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_dey() {
        use super::dey;

        let mut register = Register::new();
        register.y = 0xCC;

        let cycles = dey(&mut register);
        assert_eq!(register.y, 0xCB);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_eor() {
        use super::eor;

        let cell_in_bounds = cell(0xA5, true, 5);
        let cell_out_of_bounds = cell(0xF0, false, 5);
        let mut register = Register::new();
        register.a = 0x55;

        let cycles = eor(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0xF0);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        let cycles = eor(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_inc() {
        use super::inc;

        let cell = cell(0xCC, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = inc(&mut memory, cell, &mut register);
        assert_eq!(memory[0x02], 0xCD);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_inx() {
        use super::inx;

        let mut register = Register::new();
        register.x = 0xCC;

        let cycles = inx(&mut register);
        assert_eq!(register.x, 0xCD);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_iny() {
        use super::iny;

        let mut register = Register::new();
        register.y = 0xCC;

        let cycles = iny(&mut register);
        assert_eq!(register.y, 0xCD);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_jmp() {
        use super::jmp;

        let mut register = Register::new();
        let cell = cell(0x00, true, 5);

        let cycles = jmp(cell, &mut register);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_jsr() {
        use super::jsr;

        let mut register = Register::new();
        register.set_pc(0x600);

        let mut memory: [u8; 600] = [0x00; 600];
        let cell = cell(0x00, true, 5);

        let cycles = jsr(&mut memory, cell, &mut register);
        assert_eq!(memory[0x1ff], 0x05);
        assert_eq!(memory[0x1fe], 0xff);
        assert_eq!(register.pc(), 0x02);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_lda() {
        use super::lda;

        let mut register = Register::new();
        register.set_pc(0x600);

        let mut memory: [u8; 600] = [0x00; 600];
        let cell_in_bounds = cell(0xF2, true, 5);
        let cell_out_of_bounds = cell(0x00, false, 5);

        let cycles = lda(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0xF2);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        let cycles = lda(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_ldx() {
        use super::ldx;

        let mut register = Register::new();
        register.set_pc(0x600);

        let mut memory: [u8; 600] = [0x00; 600];
        let cell_in_bounds = cell(0xF2, true, 5);
        let cell_out_of_bounds = cell(0x00, false, 5);

        let cycles = ldx(cell_in_bounds, &mut register);
        assert_eq!(register.x, 0xF2);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        let cycles = ldx(cell_out_of_bounds, &mut register);
        assert_eq!(register.x, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_ldy() {
        use super::ldy;

        let mut register = Register::new();
        register.set_pc(0x600);

        let mut memory: [u8; 600] = [0x00; 600];
        let cell_in_bounds = cell(0xF2, true, 5);
        let cell_out_of_bounds = cell(0x00, false, 5);

        let cycles = ldy(cell_in_bounds, &mut register);
        assert_eq!(register.y, 0xF2);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        let cycles = ldy(cell_out_of_bounds, &mut register);
        assert_eq!(register.y, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_lsr() {
        use super::lsr;

        let cell_immediate = cell(0x2C, true, 0);
        let cell_in_bounds = cell(0x01, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = lsr(&mut memory, cell_immediate, &mut register);
        assert_eq!(register.a, 0x16);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);

        let cycles = lsr(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x00);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_nop() {
        use super::nop;

        let cycles = nop();
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_ora() {
        use super::ora;

        let cell_in_bounds = cell(0xA5, true, 5);
        let cell_out_of_bounds = cell(0x80, false, 5);
        let mut register = Register::new();
        register.a = 0x55;

        let cycles = ora(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0xF5);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 7);

        register.a = 0x80;
        let cycles = ora(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x80);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_pha() {
        use super::pha;

        let mut memory: [u8; 512] = [0; 512];
        let mut register = Register::new();
        register.a = 0x42;

        let cycles = pha(&mut memory, &mut register);
        assert_eq!(memory[0x01ff], 0x42);
        assert_eq!(register.s(), 0xFE);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_php() {
        use super::php;

        let mut memory: [u8; 512] = [0; 512];
        let mut register = Register::new();
        register.set_negative_bit(true);
        register.set_carry_bit(true);

        let cycles = php(&mut memory, &mut register);
        assert_eq!(memory[0x01ff], 0xB1);
        assert_eq!(register.s(), 0xFE);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b1010_0001);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_pla() {
        use super::pla;

        let mut memory: [u8; 512] = [0; 512];
        memory[0x01ff] = 0x82;
        let mut register = Register::new();
        register.push_s();

        let cycles = pla(&memory, &mut register);
        assert_eq!(register.s(), 0xFF);
        assert_eq!(register.a, 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_plp() {
        use super::plp;

        let mut memory: [u8; 512] = [0; 512];
        memory[0x01ff] = 0x82;
        let mut register = Register::new();
        register.push_s();

        let cycles = plp(&memory, &mut register);
        assert_eq!(register.s(), 0xFF);
        assert_eq!(register.p(), 0b1010_0010);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_rol() {
        use super::rol;

        let cell_immediate = cell(0x2E, true, 0);
        let cell_in_bounds = cell(0x2E, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = rol(&mut memory, cell_immediate, &mut register);
        assert_eq!(register.a, 0x5C);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);

        register.set_carry_bit(true);
        let cycles = rol(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x5D);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 9);

        let cell_in_bounds = cell(0xAE, true, 5);
        register.set_carry_bit(false);
        let cycles = rol(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x5C);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 9);

        let cell_in_bounds = cell(0x80, true, 5);
        register.set_carry_bit(false);
        let cycles = rol(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x00);
        assert_eq!(register.p(), 0b0010_0011);
        assert_eq!(cycles, 9);

        let cell_in_bounds = cell(0x41, true, 5);
        register.set_carry_bit(false);
        let cycles = rol(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_ror() {
        use super::ror;

        let cell_immediate = cell(0x2E, true, 0);
        let cell_in_bounds = cell(0x2E, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();

        let cycles = ror(&mut memory, cell_immediate, &mut register);
        assert_eq!(register.a, 0x17);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);

        register.set_carry_bit(true);
        let cycles = ror(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x97);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 9);

        let cell_in_bounds = cell(0xAD, true, 5);
        register.set_carry_bit(false);
        let cycles = ror(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x56);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 9);

        let cell_in_bounds = cell(0x00, true, 5);
        register.set_carry_bit(false);
        let cycles = ror(&mut memory, cell_in_bounds, &mut register);
        assert_eq!(memory[2], 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 9);
    }

    #[test]
    fn test_rti() {
        use super::rti;

        let mut memory: [u8; 600] = [0; 600];
        memory[0x1ff] = 0x06;
        memory[0x1fe] = 0x55;
        memory[0x1fd] = 0xA3;
        let mut register = Register::new();
        register.push_s();
        register.push_s();
        register.push_s();

        let cycles = rti(&mut memory, &mut register);
        assert_eq!(register.p(), 0b1010_0011);
        assert_eq!(register.pc(), 0x0655);
        assert_eq!(register.s(), 0xFF);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_rts() {
        use super::rts;

        let mut memory: [u8; 600] = [0; 600];
        memory[0x1ff] = 0x06;
        memory[0x1fe] = 0x55;
        let mut register = Register::new();
        register.push_s();
        register.push_s();

        let cycles = rts(&mut memory, &mut register);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(register.pc(), 0x0656);
        assert_eq!(register.s(), 0xFF);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_sbc() {
        use super::sbc;

        let cell_in_bounds = cell(0x03, true, 5);
        let cell_out_of_bounds = cell(0x03, false, 5);
        let mut register = Register::new();
        register.a = 0x45;
        register.set_carry_bit(true);

        let cycles = sbc(cell_in_bounds, &mut register);
        assert_eq!(register.a, 0x42);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 7);

        let cycles = sbc(cell_out_of_bounds, &mut register);
        assert_eq!(register.a, 0x3F);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_sec() {
        use super::sec;

        let mut register = Register::new();

        let cycles = sec(&mut register);
        assert_eq!(register.p(), 0b0010_0001);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sed() {
        use super::sed;

        let mut register = Register::new();

        let cycles = sed(&mut register);
        assert_eq!(register.p(), 0b0010_1000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sei() {
        use super::sei;

        let mut register = Register::new();

        let cycles = sei(&mut register);
        assert_eq!(register.p(), 0b0010_0100);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sta() {
        use super::sta;

        let cell = cell(0x00, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();
        register.a = 0x42;

        let cycles = sta(&mut memory, cell, &mut register);

        assert_eq!(memory[0x02], 0x42);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_stx() {
        use super::stx;

        let cell = cell(0x00, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();
        register.x = 0x42;

        let cycles = stx(&mut memory, cell, &mut register);

        assert_eq!(memory[0x02], 0x42);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_sty() {
        use super::sty;

        let cell = cell(0x00, true, 5);
        let mut memory: [u8; 3] = [0x44, 0x55, 0x66];
        let mut register = Register::new();
        register.y = 0x42;

        let cycles = sty(&mut memory, cell, &mut register);

        assert_eq!(memory[0x02], 0x42);
        assert_eq!(cycles, 7);
    }

    #[test]
    fn test_tax() {
        use super::tax;

        let mut register = Register::new();

        register.a = 0x82;
        let cycles = tax(&mut register);
        assert_eq!(register.x, 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);

        register.a = 0x00;
        let cycles = tax(&mut register);
        assert_eq!(register.x, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_tay() {
        use super::tay;

        let mut register = Register::new();

        register.a = 0x82;
        let cycles = tay(&mut register);
        assert_eq!(register.y, 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);

        register.a = 0x00;
        let cycles = tay(&mut register);
        assert_eq!(register.y, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_tsx() {
        use super::tsx;

        let mut register = Register::new();

        let cycles = tsx(&mut register);
        assert_eq!(register.x, 0xff);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_txa() {
        use super::txa;

        let mut register = Register::new();

        register.x = 0x82;
        let cycles = txa(&mut register);
        assert_eq!(register.a, 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);

        register.x = 0x00;
        let cycles = txa(&mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_txs() {
        use super::txs;

        let mut register = Register::new();

        register.x = 0x82;
        let cycles = txs(&mut register);
        assert_eq!(register.s(), 0x82);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);

        register.x = 0x00;
        let cycles = txs(&mut register);
        assert_eq!(register.s(), 0x00);
        assert_eq!(register.p(), 0b0010_0000);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_tya() {
        use super::tya;

        let mut register = Register::new();

        register.y = 0x82;
        let cycles = tya(&mut register);
        assert_eq!(register.a, 0x82);
        assert_eq!(register.p(), 0b1010_0000);
        assert_eq!(cycles, 2);

        register.y = 0x00;
        let cycles = tya(&mut register);
        assert_eq!(register.a, 0x00);
        assert_eq!(register.p(), 0b0010_0010);
        assert_eq!(cycles, 2);
    }
}
