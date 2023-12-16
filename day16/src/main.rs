#![allow(unused_mut, unused_variables, dead_code, unused_imports)]

use std::collections::HashSet;

type Beam = (isize, isize, isize, isize);

fn main() {
    let test = include_str!("data/test.txt");
    let input = include_str!("data/input.txt");

    let t1 = part_1(test);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(input);
    println!("[PART 1]: {p1}");

    let t2 = part_2(test);
    println!("[TEST 2]: {t2}");

    let p2 = part_2(input);
    println!("[PART 2]: {p2}");
}

fn part_1(input: &str) -> usize {
    make_disco(parse_input(input))((0, -1, 0, 1))
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let (h, w) = (grid.len() as isize, grid[0].len() as isize);

    let reflect = make_disco(grid);

    let r_funcs: [Box<dyn Fn(isize) -> Beam>; 2] =
        [Box::new(|x| (x, -1, 0, 1)), Box::new(|x| (x, w, 0, -1))];
    let row = r_funcs.into_iter().flat_map(|fun| (0..h).map(fun));

    let c_funcs: [Box<dyn Fn(isize) -> Beam>; 2] =
        [Box::new(|y| (-1, y, 1, 0)), Box::new(|y| (h, y, -1, 0))];
    let col = c_funcs.into_iter().flat_map(|fun| (0..w).map(fun));

    row.chain(col).map(reflect).max().unwrap()
}

fn make_disco(grid: Vec<Vec<char>>) -> impl Fn(Beam) -> usize {
    let (h, w) = (grid.len() as isize, grid[0].len() as isize);

    move |start: Beam| {
        let mut states: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        let mut positions: HashSet<(isize, isize)> = HashSet::new();

        let mut stack = vec![start];

        while let Some((mut x, mut y, mut dx, mut dy)) = stack.pop() {
            if !states.insert((x, y, dx, dy)) {
                continue;
            }

            x += dx;
            y += dy;

            if x < 0 || y < 0 || x >= h || y >= w {
                continue;
            }

            match grid[x as usize][y as usize] {
                '/' => {
                    std::mem::swap(&mut dx, &mut dy);
                    dx = -dx;
                    dy = -dy;
                }
                '\\' => {
                    std::mem::swap(&mut dx, &mut dy);
                }
                '|' if dy != 0 => {
                    stack.push((x, y, -1, 0));
                    dx = 1;
                    dy = 0;
                }
                '-' if dx != 0 => {
                    stack.push((x, y, 0, -1));
                    dx = 0;
                    dy = 1;
                }
                _ => (),
            }

            positions.insert((x, y));
            stack.push((x, y, dx, dy));
        }

        positions.len()
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
