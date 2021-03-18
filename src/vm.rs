use crate::instruction::Opcode;

pub struct VM {
    /// 32bit registers
    registers: [i32; 32],
    /// program counter
    pc: usize,
    /// vector to store a byte
    program: Vec<u8>,
    /// for DIV operation
    remainder: u32,
    /// equality check
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // We cast to usize so we can use it as an index into the array
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32; // Our registers are i32s, so we need to cast it. We'll cover that later.
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += target;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= target;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                let target = self.registers[self.next_8_bits() as usize] as usize;
                if self.equal_flag {
                    self.pc = target;
                }
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::IGL => {
                println!("IGL encountered");
                return false;
            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    // get the next 8 bits
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    // get the next 16 bits
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 15;
        test_vm.registers[1] = 10;
        test_vm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 150);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 5)
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        /// program pc+2, and jmpf register[0]=2, pc+2
        /// final pc = 4
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        /// program pc+2, and jmpb register[0]=2, pc-2
        /// final pc = 0
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_gte() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_lte() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}