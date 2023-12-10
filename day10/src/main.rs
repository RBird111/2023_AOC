use std::collections::{HashSet, VecDeque};

fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day10/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day10/src/input.txt")?;

    let t1 = part_1(&test);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    Ok(())
}

fn part_1(input: &String) -> usize {
    bfs(&parse_input(input))
}

fn bfs(matrix: &Vec<Vec<Pipe>>) -> usize {
    let mut max_len = 0;

    let (x, y) = find_start(matrix);
    let start = matrix[x][y];

    let mut queue = VecDeque::from([(start, 0, HashSet::from([start]))]);

    while let Some((pipe, len, mut visited)) = queue.pop_front() {
        println!("[pipe]: {pipe:?} {len}");
        max_len = max_len.max(len);

        if pipe == start && len > 0 {
            println!("[end]: {pipe:?}");
            return len;
        }

        let iter: Vec<_> = pipe
            .adjacent_cells()
            .into_iter()
            .filter(|&(r, c)| r < matrix.len() && c < matrix[0].len())
            .filter(|&(r, c)| matrix[r][c].is_adjacent(&pipe))
            .map(|(r, c)| matrix[r][c])
            .filter(|&pos| visited.insert(pos))
            .map(|p| (p, len + 1))
            .collect();

        queue.extend(iter.into_iter().map(|(p, l)| (p, l, visited.clone())));
    }

    max_len
}

fn find_start(matrix: &Vec<Vec<Pipe>>) -> Pos {
    matrix
        .into_iter()
        .find_map(|r| {
            r.into_iter().find_map(|p| match p {
                Pipe::Start(pos) => true.then(|| *pos),
                _ => None,
            })
        })
        .unwrap()
}

fn parse_input(input: &String) -> Vec<Vec<Pipe>> {
    input.lines().enumerate().map(parse_line).collect()
}

fn parse_line((row, line): (usize, &str)) -> Vec<Pipe> {
    line.char_indices()
        .map(|(col, s)| ((row, col), s.to_string()))
        .map(Pipe::from_str)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pipe {
    Start(Pos),
    Ground(Pos),
    Vertical(Pos),
    Horizontal(Pos),
    NorthWest(Pos),
    NorthEast(Pos),
    SouthWest(Pos),
    SouthEast(Pos),
}

impl Pipe {
    fn from_str((pos, s): (Pos, String)) -> Self {
        match s.as_str() {
            "S" => Self::Start(pos),
            "." => Self::Ground(pos),
            "|" => Self::Vertical(pos),
            "-" => Self::Horizontal(pos),
            "J" => Self::NorthWest(pos),
            "L" => Self::NorthEast(pos),
            "7" => Self::SouthWest(pos),
            "F" => Self::SouthEast(pos),
            _ => panic!("error parsing pipe"),
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        other.adjacent_cells().contains(&self.get_pos())
    }

    fn adjacent_cells(&self) -> HashSet<Pos> {
        let (north, south, east, west) = ((-1, 0), (1, 0), (0, 1), (0, -1));

        let coord = |&(x, y): &Pos, dirs: &[(i32, i32)]| {
            dirs.into_iter()
                .filter_map(|&(a, b)| {
                    let (x, y) = (a + x as i32, b + y as i32);
                    (x >= 0 && y >= 0).then(|| (x as usize, y as usize))
                })
                .collect()
        };

        match self {
            Self::Start(pos) => coord(pos, &[north, south, east, west]),
            Self::Ground(pos) => coord(pos, &[]),
            Self::Vertical(pos) => coord(pos, &[north, south]),
            Self::Horizontal(pos) => coord(pos, &[east, west]),
            Self::NorthWest(pos) => coord(pos, &[north, west]),
            Self::NorthEast(pos) => coord(pos, &[north, east]),
            Self::SouthWest(pos) => coord(pos, &[south, west]),
            Self::SouthEast(pos) => coord(pos, &[south, east]),
        }
    }

    fn get_pos(&self) -> Pos {
        *match self {
            Self::Start(pos) => pos,
            Self::Ground(pos) => pos,
            Self::Vertical(pos) => pos,
            Self::Horizontal(pos) => pos,
            Self::NorthWest(pos) => pos,
            Self::NorthEast(pos) => pos,
            Self::SouthWest(pos) => pos,
            Self::SouthEast(pos) => pos,
        }
    }
}

type Pos = (usize, usize);
