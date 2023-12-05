use std::collections::{HashSet, VecDeque};

fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day04/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day04/src/input.txt")?;

    let t1 = part_1(&test);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}\n");

    let t2 = part_2(&test);
    println!("[TEST 2]: {}", t2);

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> u32 {
    input.lines().map(count_winners).map(get_score).sum()
}

fn part_2(input: &String) -> u32 {
    input
        .lines()
        .map(count_winners)
        .scan(VecDeque::new(), get_copies)
        .sum()
}

fn get_score(count: u32) -> u32 {
    match count {
        0 => 0,
        n => 2u32.pow(n - 1),
    }
}

fn get_copies(arr: &mut VecDeque<u32>, w: u32) -> Option<u32> {
    let copies = arr.pop_front().unwrap_or(1);
    let len = (w as usize).saturating_sub(arr.len());

    arr.extend(vec![1; len]);

    for i in 0..w as usize {
        arr[i] += copies;
    }

    Some(copies)
}

fn count_winners(input: &str) -> u32 {
    let capture = re::PATTERN.captures(input).unwrap();

    let get_vals = |name: &str| capture.name(name).unwrap().as_str();
    let make_set = |name: &str| -> HashSet<u32> {
        get_vals(name)
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect()
    };

    make_set("winners").intersection(&make_set("owned")).count() as _
}

pub mod re {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref PATTERN: Regex =
            Regex::new(r"Card\s+\d+:\s+(?P<winners>(?:\d+\s*)+)\|\s+(?P<owned>(?:\d+\s*)+)")
                .expect("error parsing regex");
    }
}
