use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let mem: DynResult<Vec<Word>> = std::fs::read("day5-input.txt")?
        .split(|c| *c as char == ',')
        .map(|slice| -> DynResult<Word> { Ok(std::str::from_utf8(slice)?.parse()?) })
        .collect();
    let mem = mem?;

    let mem = mem.clone();

    let mut m = Machine::new(0, mem);
    m.execute(&mut StdIo);
    Ok(())
}

type Word = i64;

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

struct Machine {
    mem: Vec<Word>,
    ip: Word,
    decoded: (Op, [ParamMode;3]),
}

impl Machine {
    fn new(ip: Word, mem_data: Vec<Word>) -> Self {
        Self {
            ip,
            mem: mem_data,
            decoded: (Op::Halt, [ParamMode::Pointer;3]),
        }
    }

    #[inline]
    fn try_fetch(&self) -> DynResult<(Op, [ParamMode;3])> {
        let op_byte = self.read_mem_at(self.ip);
        let op_instruction = op_byte % 100;
        let param_modes = [
            ParamMode::try_from(op_byte / 100 % 10)?,
            ParamMode::try_from(op_byte / 1000 % 10)?,
            ParamMode::try_from(op_byte / 10000 % 10)?,
        ];
        let op =Op::try_from(op_instruction)?;
        Ok((op, param_modes))

    }

    #[inline]
    fn fetch(&self) -> (Op, [ParamMode;3]) {
        self.try_fetch().unwrap_or_else(|_| panic!("Invalid opcode: {}", self.read_mem_at(self.ip)))
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
    fn read_mem_at(&self, address: Word) -> Word {
        assert!(address >= 0);
        self.mem[address as usize]
    }

    fn execute(&mut self, io: &mut impl Io) {
        loop {
            self.decoded = self.fetch();
            // println!("{}: {:?}", self.read_mem_at(self.ip), self.decoded);
            match self.decoded.0 {
                Op::Add => {
                    let a = self.read(0);
                    let b = self.read(1);
                    self.write(2 , a + b);
                    self.ip += 4;
                }
                Op::Mul => {
                    let a = self.read(0);
                    let b = self.read(1);
                    self.write(2 , a * b);
                    self.ip += 4;
                }
                Op::IoRead => {
                    let input = io.read_in();
                    self.write(0 ,input);
                    self.ip += 2;
                }
                Op::IoWrite => {
                    let a = self.read(0);
                    io.write_out(a);
                    self.ip += 2;
                }
                Op::JumpIfTrue => {
                    if self.read(0) != 0 {
                        self.ip = self.read(1);
                    } else {
                        self.ip += 3;
                    }
                },
                Op::JumpIfFalse => {
                    if self.read(0) == 0 {
                        self.ip = self.read(1);
                    } else {
                        self.ip += 3;
                    }
                },
                Op::LessThan => {
                    self.write(2, (self.read(0) < self.read(1)) as _);
                    self.ip += 4;
                },
                Op::Equals => {
                    self.write(2, (self.read(0) == self.read(1)) as _);
                    self.ip += 4;
                },
                Op::Halt => break,
            }
        }
    }
}

trait Io {
    fn read_in(&mut self) -> Word;
    fn write_out(&mut self, data: Word);
}

struct StdIo;

impl Io for StdIo {
    fn read_in(&mut self) -> Word {
        let mut input = String::new();
        loop {
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input[0..input.len() - 1].parse::<Word>() {
                        Err(e) => eprintln!("Invalid input: {}. Enter valid number.", e),
                        Ok(val) => return val,
                    }
                }
                Err(error) => {
                    panic!("Error reading input: {}", error);
                },
            }
        }
    }

    fn write_out(&mut self, data: Word) {
        println!("{}", data);
    }
}

struct BufIo {
    read_pos: usize,
    input: Vec<Word>,
    output: Vec<Word>,
}

impl BufIo {
    fn new(input: Vec<Word>) -> Self {
        Self {
            read_pos: 0,
            input,
            output: Vec::new(),
        }
    }

    fn into_output(self) -> Vec<Word> {
        self.output
    }
}


impl Io for BufIo {
    fn read_in(&mut self) -> Word {
        let out = self.input[self.read_pos];
        self.read_pos += 1;
        out
    }

    fn write_out(&mut self, data: Word) {
        self.output.push(data);
    }
}

fn test_machine(prog: Vec<Word>, input: Vec<Word>) -> Vec<Word> {
    let mut m = Machine::new(0, prog);
    let mut io = BufIo::new(input);
    m.execute(&mut io);
    io.into_output()
}

#[test]
fn test_pos_mode_io() {
    let prog = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

    let out1 = test_machine(prog.clone(), vec![0]);
    let out2 = test_machine(prog.clone(), vec![10]);
    assert_eq!(out1, &[0]);
    assert_eq!(out2, &[1]);
}

#[test]
fn test_imm_mode_io() {
    let prog = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

    let out1 = test_machine(prog.clone(), vec![0]);
    let out2 = test_machine(prog.clone(), vec![10]);
    assert_eq!(out1, &[0]);
    assert_eq!(out2, &[1]);

}

#[test]
fn test_jumps_larger() {
    let prog = vec![
        3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    ];

    assert_eq!(test_machine(prog.clone(), vec![0]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![-15]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![7]), &[999]);
    assert_eq!(test_machine(prog.clone(), vec![8]), &[1000]);
    assert_eq!(test_machine(prog.clone(), vec![9]), &[1001]);

}