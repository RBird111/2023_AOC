use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    let part_1: i32 = input.lines().map(map_1).sum();
    println!("[PART 1]: {}", part_1);

    let part_2: i32 = input.lines().map(map_2).sum();
    println!("[PART 2]: {}", part_2);

    Ok(())
}

fn map_1(s: &str) -> i32 {
    let a = s.chars().find(char::is_ascii_digit).unwrap();
    let b = s.chars().rfind(char::is_ascii_digit).unwrap();

    format!("{}{}", a, b).parse().unwrap_or(0)
}

fn map_2(s: &str) -> i32 {
    lazy_static! {
        static ref PAT1: Regex =
            Regex::new(r"(?:\d|zero|one|two|three|four|five|six|seven|eight|nine)")
                .expect("error parsing regex");
        static ref PAT2: Regex =
            Regex::new(r"(?:\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
                .expect("error parsing regex");
    }

    let mut num_map: HashMap<&str, i32> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .zip(1..)
    .collect();

    num_map.extend("123456789".split("").filter(|s| s.len() > 0).zip(1..));

    let a = PAT1
        .captures_iter(s)
        .map(|c| c.extract())
        .find_map(|(n, [])| num_map.get(n))
        .unwrap();

    let b = PAT2
        .captures_iter(&s.chars().rev().collect::<String>())
        .map(|c| c.extract())
        .find_map(|(n, [])| {
            let n = n.chars().rev().collect::<String>();
            num_map.get(n.as_str())
        })
        .unwrap();

    a * 10 + b
}
