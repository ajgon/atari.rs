use super::register::Register;

#[derive(Copy, Clone, Debug)]
pub enum Addressing {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY
}

#[derive(Debug)]
pub struct MemoryCell {
    pub address: usize,
    pub value: u8,
    pub in_bounds: bool,
    pub cycles: u8,
    pub bytes: u8
}

impl Addressing {
    pub fn read(&self, memory: &[u8], register: &mut Register) -> MemoryCell {
        match self {
            Addressing::Implied => { implied() }
            Addressing::Accumulator => { accumulator(register) },
            Addressing::Immediate => {
                let value = memory[register.pc() as usize];
                register.increment_pc();

                immediate(value as usize)
            },
            Addressing::Relative => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                relative(register, address as usize)
            }
            Addressing::ZeroPage => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                zeropage(memory, address as usize)
            },
            Addressing::ZeroPageX => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                zeropage_x(memory, register, address as usize)
            },
            Addressing::ZeroPageY => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                zeropage_y(memory, register, address as usize)
            },
            Addressing::Absolute => {
                let address = memory[register.pc() as usize];
                register.increment_pc();
                let address = ((memory[register.pc() as usize] as u16) << 8) + address as u16;
                register.increment_pc();

                absolute(memory, address as usize)
            },
            Addressing::AbsoluteX => {
                let address = memory[register.pc() as usize];
                register.increment_pc();
                let address = ((memory[register.pc() as usize] as u16) << 8) + address as u16;
                register.increment_pc();

                absolute_x(memory, register, address as usize)
            },
            Addressing::AbsoluteY => {
                let address = memory[register.pc() as usize];
                register.increment_pc();
                let address = ((memory[register.pc() as usize] as u16) << 8) + address as u16;
                register.increment_pc();

                absolute_y(memory, register, address as usize)
            },
            Addressing::Indirect => {
                let address = memory[register.pc() as usize];
                register.increment_pc();
                let address = ((memory[register.pc() as usize] as u16) << 8) + address as u16;
                register.increment_pc();

                indirect(memory, address as usize)
            },
            Addressing::IndirectX => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                indirect_x(memory, register, address as usize)
            },
            Addressing::IndirectY => {
                let address = memory[register.pc() as usize];
                register.increment_pc();

                indirect_y(memory, register, address as usize)
            }
        }
    }
}

fn implied() -> MemoryCell {
    MemoryCell {
        address: 0,
        value: 0,
        in_bounds: true,
        cycles: 0,
        bytes: 0
    }
}

fn immediate(value: usize) -> MemoryCell {
    MemoryCell {
        address: value,
        value: value as u8,
        in_bounds: true,
        cycles: 0,
        bytes: 1
    }
}

fn accumulator(register: &Register) -> MemoryCell {
    MemoryCell {
        address: 0,
        value: register.a,
        in_bounds: true,
        cycles: 0,
        bytes: 0
    }
}

fn relative(register: &Register, address: usize) -> MemoryCell {
    let relative: i16 = if address > 0x7f {
        address as i16 - 0x100
    } else {
        address as i16
    };
    let address = (register.pc() as i16 + relative) as usize;
    let in_bounds = register.pc() as usize & 0xff00 == address & 0xff00;

    MemoryCell {
        address: address,
        value: 0,
        in_bounds: in_bounds,
        cycles: 1,
        bytes: 1
    }
}

fn zeropage(memory: &[u8], address: usize) -> MemoryCell {
    let address = address & 0xff;

    MemoryCell {
        address: address,
        value: memory[address],
        in_bounds: true,
        cycles: 1,
        bytes: 1
    }
}

fn zeropage_x(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let address = (address + register.x as usize) & 0xff;

    MemoryCell {
        address: address,
        value: memory[address],
        in_bounds: true,
        cycles: 2,
        bytes: 1
    }
}

fn zeropage_y(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let address = (address + register.y as usize) & 0xff;

    MemoryCell {
        address: address,
        value: memory[address],
        in_bounds: true,
        cycles: 2,
        bytes: 1
    }
}

fn absolute(memory: &[u8], address: usize) -> MemoryCell {
    let address = address & 0xffff;

    MemoryCell {
        address: address,
        value: memory[address],
        in_bounds: true,
        cycles: 2,
        bytes: 2
    }
}

fn absolute_x(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let new_address = (address + register.x as usize) & 0xffff;
    let in_bounds = new_address & 0xff00 == address & 0xff00;

    MemoryCell {
        address: new_address,
        value: memory[new_address],
        in_bounds: in_bounds,
        cycles: 3,
        bytes: 2
    }
}

fn absolute_y(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let new_address = (address + register.y as usize) & 0xffff;
    let in_bounds = new_address & 0xff00 == address & 0xff00;

    MemoryCell {
        address: new_address,
        value: memory[new_address],
        in_bounds: in_bounds,
        cycles: 3,
        bytes: 2
    }
}

fn indirect(memory: &[u8], address: usize) -> MemoryCell {
    // 6502 has a well known bug in JMP (which is the only opcode using indirect addressing).
    // When fetching indirectly new PC address, only low byte of the address is increased (and overflowing)
    // without affecting the high byte. Thus:
    // JMP $3000 - will fetch PCL from $3000, and PCH from $3001 (which is okay)
    // but
    // JMP $30FF - will fetch PCL from $30FF, but PCH will be fetched from $3000 not $3100
    // (only low byte overflows, without affecting the high one).
    let next_cell_address = ((address + 1) & 0xff) + (address & 0xff00);
    let new_address = memory[address] as usize + ((memory[next_cell_address] as usize) << 8);

    MemoryCell {
        address: new_address,
        value: 0,
        in_bounds: true,
        cycles: 4,
        bytes: 2
    }
}

fn indirect_x(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let address = (address + register.x as usize) & 0xff;
    let new_address = memory[address] as usize + ((memory[address + 1] as usize) << 8);

    MemoryCell {
        address: new_address,
        value: memory[new_address],
        in_bounds: true,
        cycles: 4,
        bytes: 1
    }
}

fn indirect_y(memory: &[u8], register: &Register, address: usize) -> MemoryCell {
    let address = memory[address & 0xff] as usize + ((memory[(address & 0xff) + 1] as usize) << 8);
    let new_address = (address + register.y as usize) & 0xffff;
    let in_bounds = new_address & 0xff00 == address & 0xff00;

    MemoryCell {
        address: new_address,
        value: memory[new_address],
        in_bounds: in_bounds,
        cycles: 4,
        bytes: 1
    }
}

pub fn stack_push(memory: &mut [u8], register: &mut Register, value: u8) {
    let stack_address:usize = register.s() as usize + 0x100;

    memory[stack_address] = value;
    register.push_s();
}

pub fn stack_pull(memory: &[u8], register: &mut Register) -> u8 {
    register.pull_s();
    let stack_address:usize = register.s() as usize + 0x100;

    memory[stack_address]
}

#[cfg(test)]
mod tests {
    use super::zeropage;
    use super::zeropage_x;
    use super::zeropage_y;
    use super::absolute;
    use super::absolute_x;
    use super::absolute_y;
    use super::indirect_x;
    use super::indirect_y;

    use crate::cpu::register::Register;

    #[test]
    fn test_zeropage() {
        let mut memory = [0; 65536];
        memory[0x30] = 0x42;

        let result = zeropage(&memory, 0x30);

        assert_eq!(result.address, 0x30);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_zeropage_x() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x35] = 0x42;
        register.x = 0x05;

        let result = zeropage_x(&memory, &register, 0x30);

        assert_eq!(result.address, 0x35);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_zeropage_x_out_of_bounds() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x35] = 0x42;
        memory[0x135] = 0x27;
        register.x = 0x36;

        let result = zeropage_x(&memory, &register, 0xff);

        assert_eq!(result.address, 0x35);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);

    }

    #[test]
    fn test_zeropage_y() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x35] = 0x42;
        register.y = 0x05;

        let result = zeropage_y(&memory, &register, 0x30);

        assert_eq!(result.address, 0x35);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_zeropage_y_out_of_bounds() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x35] = 0x42;
        memory[0x135] = 0x27;
        register.y = 0x36;

        let result = zeropage_y(&memory, &register, 0xff);

        assert_eq!(result.address, 0x35);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_absolute() {
        let mut memory = [0; 65536];

        memory[0x5a3c] = 0x42;

        let result = absolute(&memory, 0x5a3c);

        assert_eq!(result.address, 0x5a3c);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_absolute_x() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5a4c] = 0x42;
        register.x = 0x10;

        let result = absolute_x(&memory, &register, 0x5a3c);

        assert_eq!(result.address, 0x5a4c);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_absolute_x_page_boundary() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5b0c] = 0x42;
        register.x = 0x10;

        let result = absolute_x(&memory, &register, 0x5afc);

        assert_eq!(result.address, 0x5b0c);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, false);
    }

    #[test]
    fn test_absolute_x_out_of_bounds() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5a] = 0x42;
        register.x = 0x5b;

        let result = absolute_x(&memory, &register, 0xffff);

        assert_eq!(result.address, 0x5a);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, false);
    }

    #[test]
    fn test_absolute_y() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5a4c] = 0x42;
        register.y = 0x10;

        let result = absolute_y(&memory, &register, 0x5a3c);

        assert_eq!(result.address, 0x5a4c);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_absolute_y_page_boundary() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5b0c] = 0x42;
        register.y = 0x10;

        let result = absolute_y(&memory, &register, 0x5afc);

        assert_eq!(result.address, 0x5b0c);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, false);
    }

    #[test]
    fn test_absolute_y_out_of_bounds() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x5a] = 0x42;
        register.y = 0x5b;

        let result = absolute_y(&memory, &register, 0xffff);

        assert_eq!(result.address, 0x5a);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, false);
    }

    #[test]
    fn test_indirect_x() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x77] = 0x05;
        memory[0x78] = 0x01;
        memory[0x105] = 0x42;
        register.x = 0x33;

        let result = indirect_x(&memory, &register, 0x44);

        assert_eq!(result.address, 0x105);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_indirect_x_out_of_zeropage() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0xff] = 0x05;
        memory[0x100] = 0x01;
        memory[0x105] = 0x42;
        register.x = 0x33;

        let result = indirect_x(&memory, &register, 0xcc);

        assert_eq!(result.address, 0x105);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_indirect_x_overflowing_add() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x02] = 0x05;
        memory[0x03] = 0x01;
        memory[0x105] = 0x42;
        register.x = 0x36;

        let result = indirect_x(&memory, &register, 0xcc);

        assert_eq!(result.address, 0x105);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_indirect_y() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x77] = 0x05;
        memory[0x78] = 0x01;
        memory[0x109] = 0x42;
        register.y = 0x04;

        let result = indirect_y(&memory, &register, 0x77);

        assert_eq!(result.address, 0x109);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }

    #[test]
    fn test_indirect_y_out_of_bonds() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0x77] = 0xff;
        memory[0x78] = 0x01;
        memory[0x205] = 0x42;
        register.y = 0x06;

        let result = indirect_y(&memory, &register, 0x77);

        assert_eq!(result.address, 0x205);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, false);
    }

    #[test]
    fn test_indirect_y_out_of_zeropage() {
        let mut memory = [0; 65536];
        let mut register = Register::new();

        memory[0xff] = 0x05;
        memory[0x100] = 0x01;
        memory[0x109] = 0x42;
        register.y = 0x04;

        let result = indirect_y(&memory, &register, 0xff);

        assert_eq!(result.address, 0x109);
        assert_eq!(result.value, 0x42);
        assert_eq!(result.in_bounds, true);
    }
}


