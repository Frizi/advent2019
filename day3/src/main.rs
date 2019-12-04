use std::{collections::HashMap, fs};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
fn main() -> DynResult<()> {
    let input = fs::read("day3-input.txt")?;
    let mut split = input.split(|t| *t as char == '\n');

    let path1 = SegmentedPath::parse(split.next().ok_or("Missing path 1")?);
    let path2 = SegmentedPath::parse(split.next().ok_or("Missing path 2")?);

    let board = Board::from_path(&path1);

    let lowest_manhattan = board
        .intersect(&path2)
        .min_by_key(|i| i.cursor.x.abs() + i.cursor.y.abs());

    let shortest_path = board
        .intersect(&path2)
        .min_by_key(|i| i.lhs_distance + i.rhs_distance);

    println!("lowest_manhattan: {:?}", lowest_manhattan);
    println!("shortest_path: {:?}", shortest_path);

    Ok(())
}

#[derive(Debug)]
struct Board {
    map: HashMap<Cursor, usize>,
}

impl Board {
    fn from_path(path: &SegmentedPath) -> Self {
        let mut map = HashMap::new();
        for (len, cursor) in path.iter().enumerate() {
            map.entry(cursor).or_insert(len + 1);
        }
        Self { map }
    }

    fn intersect<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        path: &'b SegmentedPath,
    ) -> impl Iterator<Item = Intersection> + 'c {
        path.iter()
            .enumerate()
            .filter_map(move |(path_len, cursor)| {
                let map_len = self.map.get(&cursor)?;
                Some(Intersection {
                    cursor: cursor.clone(),
                    lhs_distance: *map_len,
                    rhs_distance: path_len + 1,
                })
            })
    }
}

#[derive(Debug)]
struct Intersection {
    cursor: Cursor,
    lhs_distance: usize,
    rhs_distance: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Cursor {
    x: isize,
    y: isize,
}

impl Cursor {
    fn single_step(&mut self, segment: &Segment) {
        match segment {
            Segment::R(_) => self.x += 1,
            Segment::L(_) => self.x -= 1,
            Segment::U(_) => self.y += 1,
            Segment::D(_) => self.y -= 1,
        };
    }
}

struct SegmentedPath {
    segments: Vec<Segment>,
}

struct PathIterator<'a> {
    path: std::slice::Iter<'a, Segment>,
    last_segment: Option<Segment>,
    cursor: Cursor,
}

impl Iterator for PathIterator<'_> {
    type Item = Cursor;
    fn next(&mut self) -> Option<Cursor> {
        loop {
            if let Some(last_segment) = self.last_segment.as_mut() {
                if last_segment.step_size() != 0 {
                    self.cursor.single_step(last_segment);
                    last_segment.mutate(|v| v - 1);
                    return Some(self.cursor.clone());
                } else {
                    self.last_segment = None;
                }
            }

            if let Some(seg) = self.path.next() {
                self.last_segment = Some(seg.clone());
                continue;
            }

            return None;
        }
    }
}

impl SegmentedPath {
    fn iter<'a>(&'a self) -> impl Iterator<Item = Cursor> + 'a {
        PathIterator {
            path: self.segments.iter(),
            last_segment: None,
            cursor: Cursor::default(),
        }
    }

    fn parse(data: &[u8]) -> SegmentedPath {
        let mut seg = None;
        let mut vec = Vec::new();

        for character in data {
            let c = *character as char;
            match c {
                ',' | '\n' => {
                    vec.extend(seg.take());
                }
                'R' | 'L' | 'U' | 'D' => {
                    if seg.is_some() {
                        panic!("Unexpected start of new segment: {}", c);
                    }
                    seg = Some(match c {
                        'R' => Segment::R(0),
                        'L' => Segment::L(0),
                        'U' => Segment::U(0),
                        'D' => Segment::D(0),
                        _ => unreachable!(),
                    });
                }
                '0'..='9' => {
                    if let Some(segment) = seg.as_mut() {
                        segment.mutate(|value| value * 10 + (c as usize - '0' as usize));
                    } else {
                        panic!("Unexpected characted for segment value: {}", c);
                    }
                }
                c => panic!("Unexpected character: {}", c),
            }
        }

        vec.extend(seg.take());
        SegmentedPath { segments: vec }
    }
}

#[derive(Debug, Clone)]
enum Segment {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

impl Segment {
    #[inline]
    fn mutate(&mut self, f: impl Fn(usize) -> usize) {
        match self {
            Segment::R(v) => *v = f(*v),
            Segment::L(v) => *v = f(*v),
            Segment::U(v) => *v = f(*v),
            Segment::D(v) => *v = f(*v),
        }
    }

    #[inline]
    fn step_size(&self) -> usize {
        match self {
            Segment::R(v) => *v,
            Segment::L(v) => *v,
            Segment::U(v) => *v,
            Segment::D(v) => *v,
        }
    }
}
