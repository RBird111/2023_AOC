#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let raw_test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day03/src/test.txt")?;
    let test = parse_input(raw_test);
    test.iter().for_each(|s| println!("{s:?}"));

    println!("[TEST 1]: {}", part_1(&test));

    let raw_input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day03/src/input.txt")?;
    let input = parse_input(raw_input);

    println!("[PART 1]: {}", part_1(&input));

    Ok(())
}

fn part_1(input: &Vec<Vec<String>>) -> i32 {
    (0..input.len())
        .map(|r| {
            (0..input[r].len())
                .map(|c| (r, c))
                .filter(|&(r, c)| input[r][c].parse::<i32>().is_ok())
                .filter_map(|(r, c)| get_part_numbers(r, c, &input))
                .collect::<HashSet<_>>()
        })
        .flatten()
        .sum()
}

fn get_part_numbers(r: usize, c: usize, arr: &Vec<Vec<String>>) -> Option<i32> {
    const NEIGHBORS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let coords_valid = |x: i32, y: i32| -> bool {
        x >= 0 && x < arr.len() as i32 && y >= 0 && y < arr[0].len() as i32
    };

    let (x0, y0) = (r as i32, c as i32);

    for (x1, y1) in NEIGHBORS {
        let (x, y) = (x0 + x1, y0 + y1);
        if coords_valid(x, y) {
            let (x, y) = (x as usize, y as usize);
            let val = arr[x][y].clone();

            if val.parse::<i32>().is_err() && val != "." {
                return arr[r][c].parse().ok();
            }
        }
    }

    None
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    let mut arr: Vec<Vec<String>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut r = 0;
    while r < arr.len() {
        let mut c = 0;
        while c < arr[0].len() {
            let mut i = c;
            let mut digits = vec![];
            while let Ok(digit) = arr[r][i].parse::<i32>() {
                digits.push(digit.to_string());
                if i + 1 == arr[0].len() {
                    break;
                }
                i += 1;
            }
            let num: String = digits.concat();
            while c < i {
                arr[r][c] = num.clone();
                c += 1;
            }
            c += 1;
        }
        r += 1;
    }

    arr
}
