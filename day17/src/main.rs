#![allow(dead_code, unused_variables, unused_mut)]

use lazy_static::lazy_static;
use parse_display::Display;

lazy_static! {
    static ref GRAPH: Vec<Vec<Block>> = parse_input(include_str!("data/test.txt"));
    static ref H: usize = GRAPH.len();
    static ref W: usize = GRAPH[0].len();
}

fn main() {
    let t1 = part_1();
    println!("[TEST 1]: {t1}");
}

fn part_1() -> usize {
    GRAPH.iter().for_each(|l| {
        l.iter().for_each(|b| print!("{b} "));
        print!("\n");
    });
    0
}

fn parse_input(input: &str) -> Vec<Vec<Block>> {
    input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.split("")
                .filter_map(|d| d.parse::<usize>().ok())
                .enumerate()
                .map(|(c, h)| ((r, c), h))
                .map(Block::new)
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Crucible {
    pos: Pos,
    dir: Direction,
    cost: usize,
    straight_moves: usize,
}

#[derive(Debug, Display, Clone, Copy)]
#[display("{heat}")]
struct Block {
    pos: Pos,
    heat: usize,
}

impl Block {
    fn new((pos, heat): ((usize, usize), usize)) -> Self {
        Self {
            pos: Pos::new(pos),
            heat,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Copy, Debug, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    #[display("↑")]
    North,

    #[display("→")]
    East,

    #[display("↓")]
    South,

    #[display("←")]
    West,
}

impl Direction {
    fn translate(&self, position: Pos) -> Option<Pos> {
        let Pos { row, col } = position;

        let pos = match self {
            Direction::North => (row.checked_sub(1)?, col),
            Direction::East => (row, (col < *W).then(|| col + 1)?),
            Direction::South => ((row < *H).then(|| row + 1)?, col),
            Direction::West => (row, col.checked_sub(1)?),
        };

        Some(Pos::new(pos))
    }

    fn get_directions(&self) -> Vec<Direction> {
        vec![self.get_left(), *self, self.get_right()]
    }

    fn get_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn get_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
