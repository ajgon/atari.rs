mod addressing;
mod alu;
mod mnemonics;
mod register;

use addressing::Addressing::Implied;
use addressing::Addressing::Accumulator;
use addressing::Addressing::Immediate;
use addressing::Addressing::Relative;
use addressing::Addressing::ZeroPage;
use addressing::Addressing::ZeroPageX;
use addressing::Addressing::ZeroPageY;
use addressing::Addressing::Absolute;
use addressing::Addressing::AbsoluteX;
use addressing::Addressing::AbsoluteY;
use addressing::Addressing::Indirect;
use addressing::Addressing::IndirectX;
use addressing::Addressing::IndirectY;

use mnemonics::Mnemonics::NUL;
use mnemonics::Mnemonics::ADC;
use mnemonics::Mnemonics::AND;
use mnemonics::Mnemonics::ASL;
use mnemonics::Mnemonics::BCC;
use mnemonics::Mnemonics::BCS;
use mnemonics::Mnemonics::BEQ;
use mnemonics::Mnemonics::BIT;
use mnemonics::Mnemonics::BMI;
use mnemonics::Mnemonics::BNE;
use mnemonics::Mnemonics::BPL;
use mnemonics::Mnemonics::BRK;
use mnemonics::Mnemonics::BVC;
use mnemonics::Mnemonics::BVS;
use mnemonics::Mnemonics::CLC;
use mnemonics::Mnemonics::CLD;
use mnemonics::Mnemonics::CLI;
use mnemonics::Mnemonics::CLV;
use mnemonics::Mnemonics::CMP;
use mnemonics::Mnemonics::CPX;
use mnemonics::Mnemonics::CPY;
use mnemonics::Mnemonics::DEC;
use mnemonics::Mnemonics::DEX;
use mnemonics::Mnemonics::DEY;
use mnemonics::Mnemonics::EOR;
use mnemonics::Mnemonics::INC;
use mnemonics::Mnemonics::INX;
use mnemonics::Mnemonics::INY;
use mnemonics::Mnemonics::JMP;
use mnemonics::Mnemonics::JSR;
use mnemonics::Mnemonics::LDA;
use mnemonics::Mnemonics::LDX;
use mnemonics::Mnemonics::LDY;
use mnemonics::Mnemonics::LSR;
use mnemonics::Mnemonics::NOP;
use mnemonics::Mnemonics::ORA;
use mnemonics::Mnemonics::PHA;
use mnemonics::Mnemonics::PHP;
use mnemonics::Mnemonics::PLA;
use mnemonics::Mnemonics::PLP;
use mnemonics::Mnemonics::ROL;
use mnemonics::Mnemonics::ROR;
use mnemonics::Mnemonics::RTI;
use mnemonics::Mnemonics::RTS;
use mnemonics::Mnemonics::SBC;
use mnemonics::Mnemonics::SEC;
use mnemonics::Mnemonics::SED;
use mnemonics::Mnemonics::SEI;
use mnemonics::Mnemonics::STA;
use mnemonics::Mnemonics::STX;
use mnemonics::Mnemonics::STY;
use mnemonics::Mnemonics::TAX;
use mnemonics::Mnemonics::TAY;
use mnemonics::Mnemonics::TSX;
use mnemonics::Mnemonics::TXA;
use mnemonics::Mnemonics::TXS;
use mnemonics::Mnemonics::TYA;


const OPCODES: [mnemonics::Mnemonics; 256] = [
    BRK(Implied),   ORA(IndirectX), NUL,            NUL, NUL,            ORA(ZeroPage),  ASL(ZeroPage),  NUL, PHP(Implied), ORA(Immediate), ASL(Accumulator), NUL, NUL,            ORA(Absolute),  ASL(Absolute),  NUL,
    BPL(Relative),  ORA(IndirectY), NUL,            NUL, NUL,            ORA(ZeroPageX), ASL(ZeroPageX), NUL, CLC(Implied), ORA(AbsoluteY), NUL,              NUL, NUL,            ORA(AbsoluteX), ASL(AbsoluteX), NUL,
    JSR(Absolute),  AND(IndirectX), NUL,            NUL, BIT(ZeroPage),  AND(ZeroPage),  ROL(ZeroPage),  NUL, PLP(Implied), AND(Immediate), ROL(Accumulator), NUL, BIT(Absolute),  AND(Absolute),  ROL(Absolute),  NUL,
    BMI(Relative),  AND(IndirectY), NUL,            NUL, NUL,            AND(ZeroPageX), ROL(ZeroPageX), NUL, SEC(Implied), AND(AbsoluteY), NUL,              NUL, NUL,            AND(AbsoluteX), ROL(AbsoluteX), NUL,
    RTI(Implied),   EOR(IndirectX), NUL,            NUL, NUL,            EOR(ZeroPage),  LSR(ZeroPage),  NUL, PHA(Implied), EOR(Immediate), LSR(Accumulator), NUL, JMP(Absolute),  EOR(Absolute),  LSR(Absolute),  NUL,
    BVC(Relative),  EOR(IndirectY), NUL,            NUL, NUL,            EOR(ZeroPageX), LSR(ZeroPageX), NUL, CLI(Implied), EOR(AbsoluteY), NUL,              NUL, NUL,            EOR(AbsoluteX), LSR(AbsoluteX), NUL,
    RTS(Implied),   ADC(IndirectX), NUL,            NUL, NUL,            ADC(ZeroPage),  ROR(ZeroPage),  NUL, PLA(Implied), ADC(Immediate), ROR(Accumulator), NUL, JMP(Indirect),  ADC(Absolute),  ROR(Absolute),  NUL,
    BVS(Relative),  ADC(IndirectY), NUL,            NUL, NUL,            ADC(ZeroPageX), ROR(ZeroPageX), NUL, SEI(Implied), ADC(AbsoluteY), NUL,              NUL, NUL,            ADC(AbsoluteX), ROR(AbsoluteX), NUL,
    NUL,            STA(IndirectX), NUL,            NUL, STY(ZeroPage),  STA(ZeroPage),  STX(ZeroPage),  NUL, DEY(Implied), NUL,            TXA(Implied),     NUL, STY(Absolute),  STA(Absolute),  STX(Absolute),  NUL,
    BCC(Relative),  STA(IndirectY), NUL,            NUL, STY(ZeroPageX), STA(ZeroPageX), STX(ZeroPageY), NUL, TYA(Implied), STA(AbsoluteY), TXS(Implied),     NUL, NUL,            STA(AbsoluteX), NUL,            NUL,
    LDY(Immediate), LDA(IndirectX), LDX(Immediate), NUL, LDY(ZeroPage),  LDA(ZeroPage),  LDX(ZeroPage),  NUL, TAY(Implied), LDA(Immediate), TAX(Implied),     NUL, LDY(Absolute),  LDA(Absolute),  LDX(Absolute),  NUL,
    BCS(Relative),  LDA(IndirectY), NUL,            NUL, LDY(ZeroPageX), LDA(ZeroPageX), LDX(ZeroPageY), NUL, CLV(Implied), LDA(AbsoluteY), TSX(Implied),     NUL, LDY(AbsoluteX), LDA(AbsoluteX), LDX(AbsoluteY), NUL,
    CPY(Immediate), CMP(IndirectX), NUL,            NUL, CPY(ZeroPage),  CMP(ZeroPage),  DEC(ZeroPage),  NUL, INY(Implied), CMP(Immediate), DEX(Implied),     NUL, CPY(Absolute),  CMP(Absolute),  DEC(Absolute),  NUL,
    BNE(Relative),  CMP(IndirectY), NUL,            NUL, NUL,            CMP(ZeroPageX), DEC(ZeroPageX), NUL, CLD(Implied), CMP(AbsoluteY), NUL,              NUL, NUL,            CMP(AbsoluteX), DEC(AbsoluteX), NUL,
    CPX(Immediate), SBC(IndirectX), NUL,            NUL, CPX(ZeroPage),  SBC(ZeroPage),  INC(ZeroPage),  NUL, INX(Implied), SBC(Immediate), NOP(Implied),     NUL, CPX(Absolute),  SBC(Absolute),  INC(Absolute),  NUL,
    BEQ(Relative),  SBC(IndirectY), NUL,            NUL, NUL,            SBC(ZeroPageX), INC(ZeroPageX), NUL, SED(Implied), SBC(AbsoluteY), NUL,              NUL, NUL,            SBC(AbsoluteX), INC(AbsoluteX), NUL
];

pub struct Cpu<'a> {
    memory: &'a mut [u8],
    register: register::Register,
    opcodes: [mnemonics::Mnemonics; 256],
    pub cycles: usize,
    debug: bool
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &mut [u8]) -> Cpu {
        Cpu {
            memory: memory,
            register: register::Register::new(),
            opcodes: OPCODES,
            cycles: 0,
            debug: false
        }
    }

    pub fn debug(&mut self) {
        self.debug = true;
    }

    pub fn cold_reset(&mut self) {
        let pc_high = self.memory[0xfffd];
        let pc_low = self.memory[0xfffc];

        let pc_high = 0x04;
        let pc_low = 0x00;
        self.register.a = 0x00;
        self.register.x = 0x00;
        self.register.y = 0x00;
        self.register.set_p(0b0010_0100); // Interrupt flag
        self.register.set_pc(((pc_high as u16) << 8) + pc_low as u16);
    }

    pub fn warm_reset(&mut self) {
        let pc_high = self.memory[0xfffd];
        let pc_low = self.memory[0xfffc];

        self.register.set_interrupt_bit(true);
        self.register.set_pc(((pc_high as u16) << 8) + pc_low as u16);
    }

    pub fn step(&mut self) -> bool {
        let pc_start = self.register.pc();
        let opcode = self.read_byte();

        let cycles = self.opcodes[opcode as usize].handle(&mut self.register, &mut self.memory) as usize;

        if self.debug {
            println!("${:x}: {:?}({:x}), A: 0x{:x}, X: 0x{:x}, Y: 0x{:x}, S: 0x01{:x}, top: 0x{:x} P: {:b}, cyc: {}", pc_start, self.opcodes[opcode as usize], opcode, self.register.a, self.register.x, self.register.y, self.register.s(), self.memory[(self.register.s().overflowing_add(1).0) as usize + 0x100 as usize], self.register.p(), cycles);
        }

        self.cycles += cycles;

        return true;
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.memory[self.register.pc() as usize];
        self.register.increment_pc();

        return byte;
    }
}
