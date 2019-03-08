/*
ADC  Add Memory to Accumulator with Carry

     A + M + C -> A, C                N Z C I D V
                                      + + + - - +

     addressing    assembler    opc  bytes  cyles
     --------------------------------------------
     immidiate     ADC #oper     69    2     2
     zeropage      ADC oper      65    2     3
     zeropage,X    ADC oper,X    75    2     4
     absolute      ADC oper      6D    3     4
     absolute,X    ADC oper,X    7D    3     4*
     absolute,Y    ADC oper,Y    79    3     4*
     (indirect,X)  ADC (oper,X)  61    2     6
     (indirect),Y  ADC (oper),Y  71    2     5*

*   16-bit address words are little endian, lo(w)-byte first, followed by the hi(gh)-byte.
(An assembler will use a human readable, big-endian notation as in $HHLL.)

*/

use crate::cpu::mnemonics::Mnemonic;
use crate::cpu::register::Register;
use crate::cpu::math::calculate;

#[derive(Debug)]
pub struct Adc {
    mnemonic: String,
    opcode: u8
}

impl Adc {
    pub fn new(opcode: u8) -> Adc {
        return Adc { mnemonic: "ADC".to_string(), opcode: opcode };
    }
}

impl Mnemonic for Adc {
    fn determine_bytes_and_cycles(&self) -> (usize, u8) {
        return match self.opcode {
            0x69 => (2, 2),
            0x65 => (2, 3),
            0x75 => (2, 4),
            0x6D => (3, 4),
            0x7D => (3, 4),
            0x79 => (3, 4),
            0x61 => (2, 6),
            0x71 => (2, 5),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call(&self, arguments: Vec<u8>, register: &mut Register) {
        match self.opcode {
            0x69 => self.call_immidiate(arguments, register),
            _ => panic!("Invalid opcode `0x{:x}` for mnemonic {}", self.opcode, self.mnemonic)
        }
    }

    fn call_immidiate(&self, arguments: Vec<u8>, register: &mut Register) {
        let carry_bit_value = if register.carry_bit() { 1 } else { 0 };
        register.set_accumulator(register.a() + arguments[0] + carry_bit_value);
        register.set_carry_bit(false);
    }
}

#[cfg(test)]
mod tests {
    use super::Adc;
    use crate::cpu::mnemonics::Mnemonic;
    use crate::cpu::register::Register;

    #[test]
    fn test_immidiate_without_carry() {
        let adc = Adc::new(0x69);
        let arguments = vec![0x42];
        let mut register = Register::new();
        register.set_accumulator(0x02);

        adc.call(arguments, &mut register);

        assert_eq!(register.a(), 0x44);
        assert_eq!(register.p(), 0b00100000);
    }

    #[test]
    fn test_immidiate_with_carry() {
        let adc = Adc::new(0x69);
        let arguments = vec![0x42];
        let mut register = Register::new();
        register.set_accumulator(0x02);
        register.set_carry_bit(true);

        adc.call(arguments, &mut register);

        assert_eq!(register.a(), 0x45);
        assert_eq!(register.p(), 0b00100000);
    }
}


