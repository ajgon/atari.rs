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
