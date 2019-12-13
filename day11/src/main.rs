use intcode::*;
use std::collections::HashMap;

fn main() -> DynResult<()> {
    let prog = parse_intcode(&std::fs::read("day11-input.txt")?)?;

    let mut robot = Robot::new();
    Machine::new(prog.clone()).execute(&mut robot);
    println!("len: {}", robot.panels.len());
    
    let mut robot = Robot::new();
    robot.panels.insert((0, 0), true);
    Machine::new(prog.clone()).execute(&mut robot);
    robot.print_drawing();    

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
            Dir::Left => Dir::Down,
        }
    }
    
    fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RobotState {
    Paint,
    Turn,
}

struct Robot {
    pos: (isize, isize),
    dir: Dir,
    state: RobotState,
    panels: HashMap<(isize, isize), bool>,
}

impl Robot {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Dir::Up,
            state: RobotState::Paint,
            panels: HashMap::new(),
        }
    }

    fn print_drawing(&self) {
        let (minx, maxx, miny, maxy) = self.panels.keys().fold(
            (isize::max_value(),
            isize::min_value(),
            isize::max_value(),
            isize::min_value()), |(minx, maxx, miny, maxy), (x, y)| {
                (
                    minx.min(*x),
                    maxx.max(*x),
                    miny.min(*y),
                    maxy.max(*y),
                )
            }
        );
    
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                let c = match self.panels.get(&(x, y)) {
                    Some(true) => "##",
                    Some(false) => ". ",
                    None => "  ",
                };
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

impl Io for Robot {
    fn read_in(&mut self) -> Option<Word> {
        Some(self.panels.get(&self.pos).cloned().unwrap_or(false) as _)
    }

    fn write_out(&mut self, data: Word) -> bool {
        match self.state {
            RobotState::Paint => {
                match data {
                    0 => self.panels.insert(self.pos, false),
                    1 => self.panels.insert(self.pos, true),
                    other => panic!("Illegal paint: {}", other),
                };
                self.state = RobotState::Turn;
            }
            RobotState::Turn => {
                match data {
                    0 => self.dir = self.dir.turn_left(),
                    1 => self.dir = self.dir.turn_right(),
                    other => panic!("Illegal turn: {}", other),
                }
                match self.dir {
                    Dir::Up => self.pos.1 += 1,
                    Dir::Down => self.pos.1 -= 1,
                    Dir::Right => self.pos.0 += 1,
                    Dir::Left => self.pos.0 -= 1,
                }

                self.state = RobotState::Paint;
            }
        };
        true
    }
}
