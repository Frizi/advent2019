use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::io::*;
use crate::DynResult;

pub type Word = i64;

#[repr(i64)]
#[derive(Debug, TryFromPrimitive, Clone, Copy)]
enum Op {
    Add = 1,
    Mul = 2,
    IoRead = 3,
    IoWrite = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[repr(i64)]
#[derive(Debug, TryFromPrimitive, Clone, Copy)]
enum ParamMode {
    Pointer = 0,
    Immediate = 1,
}

pub struct Machine {
    mem: Vec<Word>,
    ip: Word,
    decoded: (Op, [ParamMode; 3]),
}

#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    Continue,
    IoBlocked,
    Halt,
}

impl StepResult {
    pub fn join(self, other: StepResult) -> StepResult {
        use StepResult::*;
        match (self, other) {
            (Continue, _) => Continue,
            (_, Continue) => Continue,
            (IoBlocked, _) => IoBlocked,
            (_, IoBlocked) => IoBlocked,
            (Halt, Halt) => Halt,
        }
    }
}

impl Machine {
    pub fn new(mem_data: Vec<Word>) -> Self {
        Self {
            ip: 0,
            mem: mem_data,
            decoded: (Op::Halt, [ParamMode::Pointer; 3]),
        }
    }

    #[inline]
    fn try_fetch(&self) -> DynResult<(Op, [ParamMode; 3])> {
        let op_byte = self.read_mem_at(self.ip);
        let op_instruction = op_byte % 100;
        let param_modes = [
            ParamMode::try_from(op_byte / 100 % 10)?,
            ParamMode::try_from(op_byte / 1000 % 10)?,
            ParamMode::try_from(op_byte / 10000 % 10)?,
        ];
        let op = Op::try_from(op_instruction)?;
        Ok((op, param_modes))
    }

    #[inline]
    fn fetch(&self) -> (Op, [ParamMode; 3]) {
        self.try_fetch()
            .unwrap_or_else(|_| panic!("Invalid opcode: {}", self.read_mem_at(self.ip)))
    }

    #[inline]
    fn get_param(&self, param: usize) -> Word {
        let address = self.ip + param as Word + 1;
        self.read_mem_at(address)
    }

    #[inline]
    fn read(&self, param: usize) -> Word {
        let value = self.get_param(param);
        match self.decoded.1[param] {
            ParamMode::Pointer => self.read_mem_at(value),
            ParamMode::Immediate => value,
        }
    }

    #[inline]
    fn write(&mut self, param: usize, val: Word) {
        let address = self.get_param(param);
        assert!(address >= 0);
        self.mem[address as usize] = val;
    }

    #[inline]
    pub fn read_mem_at(&self, address: Word) -> Word {
        assert!(address >= 0);
        self.mem[address as usize]
    }

    pub fn execute(&mut self, io: &mut impl Io) {
        loop {
            match self.step(io) {
                StepResult::Continue => {}
                StepResult::Halt => break,
                StepResult::IoBlocked => panic!("Execution blocked on IO. IP: {}", self.ip),
            }
        }
    }

    pub fn step(&mut self, io: &mut impl Io) -> StepResult {
        self.decoded = self.fetch();
        // println!("[{}]: {:?}", self.ip, self.decoded);
        match self.decoded.0 {
            Op::Add => {
                let a = self.read(0);
                let b = self.read(1);
                self.write(2, a + b);
                self.ip += 4;
                StepResult::Continue
            }
            Op::Mul => {
                let a = self.read(0);
                let b = self.read(1);
                self.write(2, a * b);
                self.ip += 4;
                StepResult::Continue
            }
            Op::IoRead => {
                if let Some(input) = io.read_in() {
                    self.write(0, input);
                    self.ip += 2;
                    StepResult::Continue
                } else {
                    StepResult::IoBlocked
                }
            }
            Op::IoWrite => {
                let a = self.read(0);
                if io.write_out(a) {
                    self.ip += 2;
                    StepResult::Continue
                } else {
                    StepResult::IoBlocked
                }
            }
            Op::JumpIfTrue => {
                if self.read(0) != 0 {
                    self.ip = self.read(1);
                } else {
                    self.ip += 3;
                }
                StepResult::Continue
            }
            Op::JumpIfFalse => {
                if self.read(0) == 0 {
                    self.ip = self.read(1);
                } else {
                    self.ip += 3;
                }
                StepResult::Continue
            }
            Op::LessThan => {
                self.write(2, (self.read(0) < self.read(1)) as _);
                self.ip += 4;
                StepResult::Continue
            }
            Op::Equals => {
                self.write(2, (self.read(0) == self.read(1)) as _);
                self.ip += 4;
                StepResult::Continue
            }
            Op::Halt => StepResult::Halt,
        }
    }
}

#[allow(dead_code)]
fn test_machine(prog: Vec<Word>, input: Vec<Word>) -> Vec<Word> {
    let mut m = Machine::new(prog);
    let mut io = BufIo::new(input);
    m.execute(&mut io);
    io.into_output()
}

#[test]
fn test_pos_mode_io() {
    let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

    let out1 = test_machine(prog.clone(), vec![0]);
    let out2 = test_machine(prog.clone(), vec![10]);
    assert_eq!(out1, &[0]);
    assert_eq!(out2, &[1]);
}

#[test]
fn test_imm_mode_io() {
    let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

    let out1 = test_machine(prog.clone(), vec![0]);
    let out2 = test_machine(prog.clone(), vec![10]);
    assert_eq!(out1, &[0]);
    assert_eq!(out2, &[1]);
}

#[test]
fn test_jumps_larger() {
    let prog = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    assert_eq!(test_machine(prog.clone(), vec![0]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![-15]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![7]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![8]), &[1000]);
    assert_eq!(test_machine(prog.clone(), vec![9]), &[1001]);
}
