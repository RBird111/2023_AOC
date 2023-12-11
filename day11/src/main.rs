fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day11/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day11/src/input.txt")?;

    let day11_test = Day11::new(&test);
    let day11 = Day11::new(&input);

    println!("[TEST 1]: {}", day11_test.solve(1));
    println!("[PART 1]: {}", day11.solve(1));

    println!("[TEST 2]: {}", day11_test.solve(1e6 as _));
    println!("[PART 2]: {}", day11.solve(1e6 as _));

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Day11 {
    sky: Vec<Vec<Cell>>,
}

impl Day11 {
    fn new(input: &String) -> Self {
        Self {
            sky: Self::parse_input(input),
        }
    }

    fn solve(&self, growth: usize) -> usize {
        let mut sky_map = self.clone();
        sky_map.adjust_sky();
        sky_map.distance_between_galaxies(growth).into_iter().sum()
    }

    fn parse_input(input: &String) -> Vec<Vec<Cell>> {
        input
            .lines()
            .enumerate()
            .map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(|(j, c)| Cell::from_char(c, (i, j)))
                    .collect()
            })
            .collect()
    }

    fn adjust_sky(&mut self) {
        self.insert_extra_row();
        self.insert_extra_col();
        self.update_galaxies();
    }

    fn distance_between_galaxies(&self, growth: usize) -> Vec<usize> {
        let arr = self.find_galaxies();
        arr.iter()
            .enumerate()
            .flat_map(|(i, a)| {
                arr[i..]
                    .into_iter()
                    .map(|b| a.distance_between(b, growth))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn find_galaxies(&self) -> Vec<Galaxy> {
        self.sky
            .iter()
            .flat_map(|row| {
                row.into_iter()
                    .filter_map(|s| match s {
                        Cell::Object(g) => Some(g.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn update_galaxies(&mut self) {
        self.sky.iter_mut().enumerate().for_each(|(i, r)| {
            r.iter_mut()
                .enumerate()
                .for_each(|(j, s)| s.update_galaxy((i, j)))
        })
    }

    fn insert_extra_row(&mut self) {
        self.find_empty_rows()
            .into_iter()
            .enumerate()
            .for_each(|(i, r)| self.sky.insert(r + i, vec![Cell::Empty; self.sky.len()]));
    }

    fn insert_extra_col(&mut self) {
        let cols = self.find_empty_cols();
        self.sky.iter_mut().for_each(|v| {
            cols.iter()
                .enumerate()
                .for_each(|(i, &c)| v.insert(c + i, Cell::Empty))
        });
    }

    fn find_empty_rows(&self) -> Vec<usize> {
        self.sky
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.into_iter().all(|&c| c == Cell::Empty).then(|| i))
            .collect()
    }

    fn find_empty_cols(&self) -> Vec<usize> {
        (0..self.sky[0].len())
            .filter_map(|i| {
                self.sky
                    .iter()
                    .map(|r| r[i])
                    .all(|c| c == Cell::Empty)
                    .then(|| i)
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Galaxy {
    id: usize,
    coords: (usize, usize),
    base_coords: (usize, usize),
    updated_coords: (usize, usize),
}

impl Galaxy {
    fn new(coords: (usize, usize)) -> Self {
        Self {
            id: Self::get_id(),
            coords,
            base_coords: coords,
            updated_coords: (0, 0),
        }
    }

    fn distance_between(&self, other: &Galaxy, growth: usize) -> usize {
        let (a, b) = self.base_distance_between(other);
        let (c, d) = self.updated_distance_between(other);
        let (da, db) = ((c - a) * (growth - 1).max(1), (d - b) * (growth - 1).max(1));
        a + da + b + db
    }

    fn base_distance_between(&self, other: &Galaxy) -> (usize, usize) {
        let (x0, y0) = self.base_coords;
        let (x1, y1) = other.base_coords;
        (x0.abs_diff(x1), y0.abs_diff(y1))
    }

    fn updated_distance_between(&self, other: &Galaxy) -> (usize, usize) {
        let (x0, y0) = self.updated_coords;
        let (x1, y1) = other.updated_coords;
        (x0.abs_diff(x1), y0.abs_diff(y1))
    }

    fn update_loc(&mut self, coords: (usize, usize)) {
        self.coords = coords;
        self.updated_coords = coords;
    }

    fn get_id() -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static COUNTER: AtomicUsize = AtomicUsize::new(1);

        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Object(Galaxy),
}

impl Cell {
    fn from_char(c: char, coords: (usize, usize)) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Object(Galaxy::new(coords)),
            _ => panic!("Sky::from_char -- error parsing character"),
        }
    }

    fn update_galaxy(&mut self, coords: (usize, usize)) {
        match self {
            Self::Object(g) => g.update_loc(coords),
            _ => (),
        }
    }
}
