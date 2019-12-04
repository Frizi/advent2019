use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

type Word = u32;

fn main() -> DynResult<()> {
    let mem: DynResult<Vec<Word>> = std::fs::read("input-day2.txt")?
        .split(|c| *c as char == ',')
        .map(|slice| -> DynResult<Word> { Ok(std::str::from_utf8(slice)?.parse()?) })
        .collect();
    let mem = mem?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = mem.clone();
            mem[1] = noun;
            mem[2] = verb;
        
            let mut m = Machine::new(0, mem);
            m.execute();

            let output = m.mem[0];

            if output == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                return Ok(())
            }
        }
    }

    Ok(())
}

#[repr(u32)]
#[derive(TryFromPrimitive)]
enum Op {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

struct Machine {
    mem: Vec<Word>,
    ip: usize,
}

impl Machine {
    fn new(ip: usize, mem: Vec<Word>) -> Self {
        Self { ip, mem }
    }

    fn get_op(&self) -> Op {
        let op_byte = self.mem[self.ip];
        Op::try_from(op_byte).unwrap_or_else(|_| panic!("Invalid opcode: {}", op_byte))
    }

    fn get_param(&self, param: Word) -> Word {
        self.mem[self.ip + param  as usize + 1]
    }

    fn read(&self, addr: Word) -> Word {
        self.mem[addr as usize]
    }

    fn write(&mut self, addr: Word, val: Word) {
        self.mem[addr as usize] = val
    }

    fn execute(&mut self) {
        loop {
            match self.get_op() {
                Op::Add => {
                    let a = self.read(self.get_param(0) );
                    let b = self.read(self.get_param(1) );
                    self.write(self.get_param(2) , a + b);
                }
                Op::Mul => {
                    let a = self.read(self.get_param(0) );
                    let b = self.read(self.get_param(1) );
                    self.write(self.get_param(2) , a * b);
                }
                Op::Halt => break,
            }
            self.ip += 4;
        }
    }
}
