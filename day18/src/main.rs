#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let test = include_str!("data/test.txt");
    let input = include_str!("data/input.txt");

    let t1 = part_1(test);
    // println!("[TEST 1]: {t1}");

    let p1 = part_1(input);
    // println!("[PART 1]: {p1}");

    make_grid(input).iter().for_each(|l| {
        l.iter().for_each(|s| print!("{s}"));
        print!("\n");
    })
}

fn part_1(input: &str) -> usize {
    find_area(&make_grid(input))
}

fn find_area(grid: &Vec<Vec<&str>>) -> usize {
    grid.into_iter()
        .map(|row| {
            let left = (0..row.len()).find(|&i| row[i] == "#").unwrap();
            let right = (0..row.len()).rfind(|&i| row[i] == "#").unwrap();
            right - left + 1
        })
        .sum()
}

fn make_grid(input: &str) -> Vec<Vec<&str>> {
    use std::collections::HashSet;

    let mut r = 0;
    let mut rows = 0;

    let mut c = 0;
    let mut cols = 0;

    let mut points = HashSet::new();

    input
        .lines()
        .map(|l| l.split_ascii_whitespace().take(2).collect::<Vec<_>>())
        .for_each(|v| {
            let (dir, dist) = (v[0], v[1]);
            let dist: isize = dist.parse().unwrap();

            let (or, oc) = (r, c);

            match dir {
                "L" => c -= dist,
                "U" => r -= dist,
                "R" => c += dist,
                "D" => r += dist,
                _ => (),
            }

            if c == oc {
                let a = r.min(or);
                let b = r.max(or);

                (a..b + 1).for_each(|r| {
                    points.insert((r, c));
                })
            } else {
                let a = c.min(oc);
                let b = c.max(oc);

                (a..b + 1).for_each(|c| {
                    points.insert((r, c));
                })
            }

            points.insert((r, c));

            rows = rows.max(r);
            cols = cols.max(c);
        });

    let (mx, my) = points
        .iter()
        .fold((isize::MAX, isize::MAX), |(x, y), &(a, b)| {
            (x.min(a), y.min(b))
        });
    let points: HashSet<_> = points
        .into_iter()
        .map(|(x, y)| (x + mx.abs(), y + my.abs()))
        .collect();

    (0..rows + mx.abs() + 1)
        .map(|r| {
            (0..cols + my.abs() + 1)
                .map(|c| if points.contains(&(r, c)) { "#" } else { "." })
                .collect()
        })
        .collect()
}
