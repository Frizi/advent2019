use intcode::*;
use std::collections::VecDeque;

fn main() -> DynResult<()> {
    let mut prog = parse_intcode(&std::fs::read("day12-input.txt")?)?;

    macro_rules! tas {
        (@+$x:tt) => { TasInput::Right($x) };
        (@-$x:tt) => { TasInput::Left($x) };
        (@!$x:tt) => { TasInput::Wait($x) };
        ($($dir:tt $item:tt),*,) => { &[$(tas![@ $dir $item]),*] };
    }

    let mut game = Game::new(tas![
        +1, !9, -9, !10, +13, !20,
        -2, !416, -20, !350, +12, !120,
        -12, !313, +32, -31, !10, -1,
        !10, +3, !5, +8, !20, +3,
        !79, -14, +34, !294, -24, !30,
        -7, !217, +26, !70, -26, !45,
        -1, !10, +29, !3, -6, !58,
        -8, +1, !180, -3, !20, +3,
        !50, -3, !10, -1, !110, +5,
        !35, -6, !30, +8, !30, +1,
        !30, -11, !80, -1, !35, +13,
        !2, -14, !80, +17, !66,
    ]);

    Machine::new(prog.clone()).execute(&mut game);
    println!("Total of blocks: {}", game.count_tile(Tile::Block));
    prog[0] = 2;
    Machine::new(prog).execute(&mut game);
    println!("Score at end: {}", game.score);
    Ok(())
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl Tile {
    fn glyph(self) -> &'static str {
        match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "█",
            Tile::HorizontalPaddle => "-",
            Tile::Ball => "o",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TasInput {
    Left(usize),
    Right(usize),
    Wait(usize),
}

impl TasInput {
    fn step(self) -> (Option<Self>, Word) {
        match self {
            Self::Left(x) if x <= 1 => (None, -1),
            Self::Right(x) if x <= 1 => (None, 1),
            Self::Wait(x) if x <= 1 => (None, 0),
            Self::Left(x) => (Some(TasInput::Left(x - 1)), -1),
            Self::Right(x) => (Some(TasInput::Right(x - 1)), 1),
            Self::Wait(x) => (Some(TasInput::Wait(x - 1)), 0),
        }
    }
}

struct Game {
    board: [[Tile; 64]; 64],
    board_max: (usize, usize),
    score: usize,
    io_buffer: VecDeque<Word>,
    tas_input: Vec<TasInput>,
}

impl Io for Game {
    fn read_in(&mut self) -> Option<Word> {
        if let Some(tas_input) = self.tas_input.pop() {
            let (steps_left, step) = tas_input.step();
            self.tas_input.extend(steps_left);
            Some(step)
        } else {
            self.draw();
            StdIo.read_in()
        }
    }

    fn write_out(&mut self, data: Word) -> bool {
        self.io_buffer.push_back(data);
        while self.io_buffer.len() >= 3 {
            let a = self.io_buffer.pop_front().unwrap();
            let b = self.io_buffer.pop_front().unwrap();
            let c = self.io_buffer.pop_front().unwrap();

            if a == -1 && b == 0 {
                self.score = c as usize;
            } else {
                let x = a as usize;
                let y = b as usize;
                let tile = match c {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::HorizontalPaddle,
                    4 => Tile::Ball,
                    t => panic!("Unknown tile {}", t),
                };

                self.board[y][x] = tile;
                self.board_max.0 = self.board_max.0.max(x);
                self.board_max.1 = self.board_max.1.max(y);
            }
        }
        true
    }
}

impl Game {
    fn new(input: &[TasInput]) -> Self {
        Self {
            board: [[Tile::Empty; 64]; 64],
            board_max: (0, 0),
            score: 0,
            io_buffer: VecDeque::new(),
            tas_input: input.into_iter().rev().cloned().collect(),
        }
    }
    fn draw(&self) {
        for y in 0..self.board_max.1 {
            for x in 0..self.board_max.0 {
                let tile = &self.board[y][x];
                print!("{}", tile.glyph());
            }
            print!("\n");
        }
        println!("Score: {}", self.score);
    }

    fn count_tile(&self, tile: Tile) -> usize {
        self.board
            .iter()
            .map(|row| row.iter().filter(|t| **t == tile).count())
            .sum()
    }
}
