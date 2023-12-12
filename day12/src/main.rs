use rayon::prelude::*;

use std::collections::HashMap;

type Cache = HashMap<(usize, usize, usize), usize>;

fn main() {
    let test = include_str!("test.txt");
    let input = include_str!("input.txt");

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
    parse_input(input)
        .par_iter()
        .cloned()
        .map(total_combinations)
        .sum()
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .par_iter()
        .map(expand)
        .map(total_combinations)
        .sum()
}

fn total_combinations((springs, groups): (Vec<char>, Vec<usize>)) -> usize {
    let mut combos: Cache = HashMap::new();
    combinations((0, 0, 0), &mut combos, &springs, &groups)
}

fn combinations(
    tup: (usize, usize, usize),
    combos: &mut Cache,
    springs: &Vec<char>,
    groups: &Vec<usize>,
) -> usize {
    let cache = |combos: &mut Cache| {
        let (i, j, r) = tup;
        if j == groups.len() {
            return (springs[i..].into_iter().filter(|&&c| c == '#').count() == 0) as _;
        }

        if i == springs.len() {
            return (j == groups.len() - 1 && r == groups[j]) as _;
        }

        let mut n = 0;
        if springs[i] != '.' {
            n += combinations((i + 1, j, r + 1), combos, springs, groups);
        }

        if springs[i] != '#' {
            if r == groups[j] {
                n += combinations((i + 1, j + 1, 0), combos, springs, groups);
            } else if r == 0 {
                n += combinations((i + 1, j, 0), combos, springs, groups);
            }
        }

        combos.insert(tup, n);
        n
    };

    match combos.get(&tup) {
        Some(&v) => v,
        None => cache(combos),
    }
}

fn expand((s, g): &(Vec<char>, Vec<usize>)) -> (Vec<char>, Vec<usize>) {
    (
        vec![s.into_iter().collect::<String>(); 5]
            .join("?")
            .chars()
            .collect::<Vec<_>>(),
        vec![g.clone(); 5].concat(),
    )
}

fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (s, v) = line.split_once(" ").unwrap();
    (
        s.chars().collect(),
        v.split(",").filter_map(|c| c.parse().ok()).collect(),
    )
}
