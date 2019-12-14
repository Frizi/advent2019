type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let input = std::fs::read("day10-input.txt")?;
    let map = parse_map(&input)?;
    println!("{:?}", best_location(&map));
    Ok(())
}

fn parse_map(map: &[u8]) -> DynResult<Map> {
    let first = map.iter().position(|c| *c == '.' as u8 || *c == '#' as u8);
    let last = map.iter().rposition(|c| *c == '.' as u8 || *c == '#' as u8);

    let useful_map = match (first, last) {
        (Some(first), Some(last)) => &map[first..last + 1],
        _ => &[],
    };

    let width = useful_map
        .iter()
        .position(|c| *c == '\n' as u8)
        .unwrap_or(1) as u32;

    let data: Vec<Cell> = useful_map
        .iter()
        .filter_map(|c| match *c as char {
            '#' => Some(Cell::Asteroid),
            '.' => Some(Cell::Empty),
            _ => None,
        })
        .collect();

    let height = data.len() as u32 / width;

    Ok(Map {
        width,
        height,
        data,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Loc {
    pos: Pos,
    score: usize,
}

fn best_location(map: &Map) -> Option<Loc> {
    map.asteroids()
        .map(|pos| {
            let score = map
                .asteroids()
                .filter(|pos2| check_visibility(map, pos, *pos2))
                .count();
            Loc { pos, score }
        })
        .max_by_key(|l| l.score)
}

fn check_visibility(map: &Map, from: Pos, to: Pos) -> bool {
    if from == to {
        return false;
    }

    let dx = to.x as i32 - from.x as i32;
    let dy = to.y as i32 - from.y as i32;

    let denom = gcd(dx, dy).max(1);
    let ddx = dx / denom;
    let ddy = dy / denom;
    for multiple in 1..denom {
        let x = (from.x as i32 + ddx * multiple) as u32;
        let y = (from.y as i32 + ddy * multiple) as u32;
        if map.is_occupied(x, y) {
            return false;
        }
    }
    return true;
}

#[inline]
fn gcd(mut m: i32, mut n: i32) -> i32 {
    if m == 0 || n == 0 {
        return (m | n).abs();
    }
    let shift: u32 = (m | n).trailing_zeros();
    if m == i32::min_value() || n == i32::min_value() {
        return (1 << shift) as i32;
    }

    m = m.abs();
    n = n.abs();

    n >>= n.trailing_zeros();
    while m != 0 {
        m >>= m.trailing_zeros();
        if n > m {
            std::mem::swap(&mut n, &mut m)
        }
        m -= n;
    }
    n << shift
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Asteroid,
}

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    data: Vec<Cell>,
}

impl Map {
    fn positions(&self) -> impl Iterator<Item = Pos> {
        let w = self.width;
        let h = self.height;
        (0..w).flat_map(move |x| (0..h).map(move |y| Pos { x, y }))
    }

    fn asteroids<'a>(&'a self) -> impl Iterator<Item = Pos> + 'a {
        self.positions()
            .filter(move |pos| self.is_occupied(pos.x, pos.y))
    }

    fn is_occupied(&self, x: u32, y: u32) -> bool {
        self.data[(x + y * self.width) as usize] == Cell::Asteroid
    }
}

#[test]
fn test_example1() -> DynResult<()> {
    let map = parse_map(
        r#"
        .#..#
        .....
        #####
        ....#
        ...##"#
            .as_bytes(),
    )?;
    assert_eq!(
        best_location(&map),
        Some(Loc {
            pos: Pos { x: 3, y: 4 },
            score: 8
        })
    );
    Ok(())
}

#[test]
fn test_example2() -> DynResult<()> {
    let map = parse_map(
        r#"
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####"#
            .as_bytes(),
    )?;
    assert_eq!(
        best_location(&map),
        Some(Loc {
            pos: Pos { x: 5, y: 8 },
            score: 33
        })
    );
    Ok(())
}

#[test]
fn test_example3() -> DynResult<()> {
    let map = parse_map(
        r#"
        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###."#
            .as_bytes(),
    )?;
    assert_eq!(
        best_location(&map),
        Some(Loc {
            pos: Pos { x: 1, y: 2 },
            score: 35
        })
    );
    Ok(())
}

#[test]
fn test_example4() -> DynResult<()> {
    let map = parse_map(
        r#"
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#.."#
            .as_bytes(),
    )?;
    assert_eq!(
        best_location(&map),
        Some(Loc {
            pos: Pos { x: 6, y: 3 },
            score: 41
        })
    );
    Ok(())
}

#[test]
fn test_example5() -> DynResult<()> {
    let map = parse_map(
        r#"
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##"#
            .as_bytes(),
    )?;
    assert_eq!(
        best_location(&map),
        Some(Loc {
            pos: Pos { x: 11, y: 13 },
            score: 210
        })
    );
    Ok(())
}
