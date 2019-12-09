use crate::machine::Word;
use std::collections::VecDeque;

pub trait Io {
    fn read_in(&mut self) -> Option<Word>;
    fn write_out(&mut self, data: Word) -> bool;
}

pub struct StdIo;

impl Io for StdIo {
    fn read_in(&mut self) -> Option<Word> {
        let mut input = String::new();
        loop {
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let crlf = input
                        .chars()
                        .position(|c| c == '\n' || c == '\r')
                        .unwrap_or(input.len());

                    match input[0..crlf].parse::<Word>() {
                        Err(e) => eprintln!("Invalid input: {}. Enter valid number.", e),
                        Ok(val) => return Some(val),
                    }
                }
                Err(error) => {
                    panic!("Error reading input: {}", error);
                }
            }
        }
    }

    fn write_out(&mut self, data: Word) -> bool {
        println!("{}", data);
        true
    }
}

pub struct IoBuffer {
    inner: VecDeque<Word>,
}

impl IoBuffer {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn with_data(data: &[Word]) -> Self {
        Self {
            inner: data.iter().copied().collect(),
        }
    }

    pub fn into_inner(self) -> VecDeque<Word> {
        self.inner
    }
}

pub struct PipedIo<'a> {
    read_buf: &'a mut IoBuffer,
    write_buf: &'a mut IoBuffer,
}

impl<'a> PipedIo<'a> {
    pub fn new(read_buf: &'a mut IoBuffer, write_buf: &'a mut IoBuffer) -> Self {
        Self {
            read_buf,
            write_buf,
        }
    }
}

impl Io for PipedIo<'_> {
    fn read_in(&mut self) -> Option<Word> {
        self.read_buf.inner.pop_front()
    }

    fn write_out(&mut self, data: Word) -> bool {
        self.write_buf.inner.push_back(data);
        true
    }
}

pub struct BufIo {
    read_pos: usize,
    input: Vec<Word>,
    output: Vec<Word>,
}

impl BufIo {
    pub fn new(input: Vec<Word>) -> Self {
        Self {
            read_pos: 0,
            input,
            output: Vec::new(),
        }
    }

    pub fn into_output(self) -> Vec<Word> {
        self.output
    }
}

impl Io for BufIo {
    fn read_in(&mut self) -> Option<Word> {
        let out = self.input[self.read_pos];
        self.read_pos += 1;
        Some(out)
    }

    fn write_out(&mut self, data: Word) -> bool {
        self.output.push(data);
        true
    }
}
