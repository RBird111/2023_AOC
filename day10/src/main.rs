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
    let start = find_start(matrix);

    let mut visited = HashSet::from([(start.x, start.y)]);
    let mut queue = VecDeque::from([(start.clone(), 0)]);

    let mut max_len = 0;

    while let Some((pipe, len)) = queue.pop_front() {
        max_len = len;

        pipe.find_adjacent(matrix)
            .into_iter()
            // .inspect(|p| println!("[pipe]: {p:?}"))
            .filter(|p| visited.insert((p.x, p.y)))
            .for_each(|p| queue.push_back((p, len + 1)));
    }

    max_len
}

fn find_start(matrix: &Vec<Vec<Pipe>>) -> Pipe {
    matrix
        .into_iter()
        .find_map(|row| row.into_iter().find(|p| p.shape == PipeShape::Start))
        .map(|p| p.clone())
        .unwrap()
}

fn parse_input(input: &String) -> Vec<Vec<Pipe>> {
    let lines: Vec<_> = input.lines().collect();
    let grid_size = (lines.len() as i32, lines[0].len() as i32);
    let parse_line = make_parse_line(grid_size);

    lines.into_iter().enumerate().map(parse_line).collect()
}

fn make_parse_line(grid_size: (i32, i32)) -> impl Fn((usize, &str)) -> Vec<Pipe> {
    move |(row, line): (usize, &str)| {
        line.char_indices()
            .map(|(col, c)| Pipe::from_char(c, (row, col), grid_size))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pipe {
    x: usize,
    y: usize,
    m: i32,
    n: i32,
    shape: PipeShape,
}

impl Pipe {
    fn from_char(c: char, (x, y): (usize, usize), (m, n): (i32, i32)) -> Self {
        Self {
            x,
            y,
            m,
            n,
            shape: PipeShape::from_char(c),
        }
    }

    fn find_adjacent(&self, matrix: &Vec<Vec<Pipe>>) -> HashSet<Pipe> {
        let valid_shapes = self.shape.connects_to();
        let (north, south, east, west) = ((-1, 0), (1, 0), (0, 1), (0, -1));

        let get_adj = |dirs: &[(i32, i32)]| {
            let (x, y) = (self.x as i32, self.y as i32);
            dirs.into_iter()
                .map(|(a, b)| (a + x, b + y))
                .filter(|&(a, b)| a >= 0 && a < self.m && b >= 0 && b < self.n)
                .map(|(r, c)| (r as usize, c as usize))
                .map(|(r, c)| matrix[r][c].clone())
                // .inspect(|p| println!("{p:?}"))
                .filter(|p| valid_shapes.contains(&p.shape))
                .collect()
        };

        match self.shape {
            PipeShape::Start => get_adj(&[north, south, east, west]),
            PipeShape::Ground => get_adj(&[]),
            PipeShape::Horizontal => get_adj(&[east, west]),
            PipeShape::Vertical => get_adj(&[north, south]),
            PipeShape::NorthWest => get_adj(&[north, west]),
            PipeShape::NorthEast => get_adj(&[north, east]),
            PipeShape::SouthWest => get_adj(&[south, west]),
            PipeShape::SouthEast => get_adj(&[south, east]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PipeShape {
    Start,
    Ground,
    Horizontal,
    Vertical,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl PipeShape {
    fn from_char(c: char) -> Self {
        use PipeShape::*;

        match c {
            'S' => Start,
            '.' => Ground,
            '-' => Horizontal,
            '|' => Vertical,
            'J' => NorthWest,
            'L' => NorthEast,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => panic!("error parsing PipeShape"),
        }
    }

    fn connects_to(&self) -> HashSet<PipeShape> {
        use PipeShape::*;

        match self {
            Start => vec![
                Vertical, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest,
            ],
            Ground => vec![],
            Horizontal => vec![Horizontal, NorthEast, NorthWest, SouthEast, SouthWest],
            Vertical => vec![Vertical, NorthEast, NorthWest, SouthEast, SouthWest],
            NorthWest => vec![Vertical, Horizontal, NorthEast, SouthEast, SouthWest],
            NorthEast => vec![Vertical, Horizontal, NorthWest, SouthEast, SouthWest],
            SouthWest => vec![Vertical, Horizontal, NorthWest, SouthEast, NorthEast],
            SouthEast => vec![Vertical, Horizontal, NorthWest, SouthWest, NorthEast],
        }
        .into_iter()
        .collect()
    }
}
