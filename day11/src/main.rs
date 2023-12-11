fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day11/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day11/src/input.txt")?;

    let t1 = part_1(&test);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    Ok(())
}

fn part_1(input: &String) -> usize {
    let sky = adjust_arr(parse_input(input));
    distance_between_galaxies(&sky).into_iter().sum()
}

fn parse_input(input: &String) -> Vec<Vec<Sky>> {
    input.lines().map(parse_row).collect()
}

fn adjust_arr(arr: Vec<Vec<Sky>>) -> Vec<Vec<Sky>> {
    let rows = find_empty_rows(&arr);
    let cols = find_empty_cols(&arr);
    update_galaxies(insert_extra_cols(cols, &insert_extra_rows(rows, &arr)))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Galaxy {
    id: usize,
    coords: (usize, usize),
}

impl Galaxy {
    fn new() -> Self {
        Self {
            id: Self::get_id(),
            coords: (0, 0),
        }
    }

    fn distance_between(&self, other: &Galaxy) -> usize {
        let (x0, y0) = self.coords;
        let (x1, y1) = other.coords;
        x0.abs_diff(x1) + y0.abs_diff(y1)
    }

    fn update_loc(&mut self, coords: (usize, usize)) {
        self.coords = coords;
    }

    fn get_id() -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static COUNTER: AtomicUsize = AtomicUsize::new(1);

        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Sky {
    Empty,
    Object(Galaxy),
}

impl Sky {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Sky::Empty,
            '#' => Sky::Object(Galaxy::new()),
            _ => panic!("Sky::from_char -- error parsing character"),
        }
    }
}

impl std::fmt::Display for Sky {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "⬛",
                Self::Object(_) => "🟦",
            }
        )
    }
}

// Helpers
fn parse_row(row: &str) -> Vec<Sky> {
    row.chars().map(Sky::from_char).collect()
}

fn distance_between_galaxies(arr: &Vec<Vec<Sky>>) -> Vec<usize> {
    let arr = find_galaxies(arr);

    let mut ans = vec![];
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            ans.push(arr[i].distance_between(&arr[j]))
        }
    }
    ans
}

fn find_galaxies(arr: &Vec<Vec<Sky>>) -> Vec<Galaxy> {
    arr.into_iter()
        .flat_map(|row| {
            row.into_iter()
                .filter_map(|s| match s {
                    Sky::Object(g) => Some(g.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn update_galaxies(arr: Vec<Vec<Sky>>) -> Vec<Vec<Sky>> {
    arr.into_iter()
        .enumerate()
        .map(|(i, r)| {
            r.into_iter()
                .enumerate()
                .map(|(j, s)| match s {
                    Sky::Object(mut g) => {
                        g.update_loc((i, j));
                        Sky::Object(g)
                    }
                    Sky::Empty => Sky::Empty,
                })
                .collect()
        })
        .collect()
}

fn insert_extra_rows(rows: Vec<usize>, arr: &Vec<Vec<Sky>>) -> Vec<Vec<Sky>> {
    arr.into_iter()
        .map(|v| vec![v.clone()])
        .enumerate()
        .flat_map(|(i, mut v)| {
            if rows.contains(&i) {
                v.push(v[0].clone())
            }
            v
        })
        .collect()
}

fn insert_extra_cols(cols: Vec<usize>, arr: &Vec<Vec<Sky>>) -> Vec<Vec<Sky>> {
    let mut arr = arr.clone();
    arr.iter_mut().for_each(|v| {
        cols.iter()
            .enumerate()
            .for_each(|(i, &c)| v.insert(c + i, Sky::Empty))
    });
    arr
}

fn find_empty_rows(arr: &Vec<Vec<Sky>>) -> Vec<usize> {
    arr.into_iter()
        .enumerate()
        .filter_map(|(i, r)| r.into_iter().all(|&c| c == Sky::Empty).then(|| i))
        .collect()
}

fn find_empty_cols(arr: &Vec<Vec<Sky>>) -> Vec<usize> {
    (0..arr[0].len())
        .filter_map(|i| {
            arr.into_iter()
                .map(|r| r[i])
                .all(|c| c == Sky::Empty)
                .then(|| i)
        })
        .collect()
}

// fn disp_arr(arr: &Vec<Vec<Sky>>) {
//     arr.into_iter().for_each(disp_row)
// }

// fn disp_row(r: &Vec<Sky>) {
//     r.into_iter().for_each(|s| print!("{s}"));
//     print!("\n")
// }

// fn print_distances(arr: &Vec<Vec<Sky>>) {
//     let arr = find_galaxies(arr);
//
//     for i in 0..arr.len() {
//         for j in (i + 1)..arr.len() {
//             println!(
//                 "{} to {}: {}",
//                 i + 1,
//                 j + 1,
//                 arr[i].distance_between(&arr[j])
//             );
//         }
//     }
// }
