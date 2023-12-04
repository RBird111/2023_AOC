use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day02/src/test.txt")?;
    let test: i32 = test
        .lines()
        .map(to_color_map)
        .filter(is_possible)
        .map(to_id)
        .sum();
    println!("[TEST]: {test}");

    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day02/src/input.txt")?;

    let part_1: i32 = input
        .lines()
        .map(to_color_map)
        .filter(is_possible)
        .map(to_id)
        .sum();
    println!("[PART 1]: {part_1}");

    let part_2: i32 = input.lines().map(to_color_map).map(get_power).sum();
    println!("[PART 2]: {part_2}");

    Ok(())
}

fn to_color_map(line: &str) -> HashMap<&str, i32> {
    lazy_static! {
        static ref ID_PAT: Regex = Regex::new(r"(?P<id>\d+):").expect("error parsing regex");
        static ref COLOR_PAT: Regex =
            Regex::new(r"(?P<green>\d+) green|(?P<red>\d+) red|(?P<blue>\d+) blue")
                .expect("error parsing regex");
    };

    let colors = ["red", "green", "blue"];
    let mut color_map: HashMap<&str, i32> = colors.iter().map(|&c| (c, 0)).collect();

    let id: i32 = ID_PAT
        .captures(line)
        .unwrap()
        .name("id")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    color_map.insert("id", id);

    COLOR_PAT.captures_iter(line).for_each(|cap| {
        colors.iter().for_each(|&color| {
            if let Some(mat) = cap.name(color) {
                let count: i32 = mat.as_str().parse().unwrap();
                color_map.entry(color).and_modify(|v| *v = (*v).max(count));
            }
        })
    });

    color_map
}

fn is_possible(map: &HashMap<&str, i32>) -> bool {
    [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .all(|(color, count)| map[color] <= count)
}

fn to_id(map: HashMap<&str, i32>) -> i32 {
    map["id"]
}

fn get_power(map: HashMap<&str, i32>) -> i32 {
    map.into_iter()
        .filter(|&(k, _)| k != "id")
        .map(|(_, v)| v)
        .product()
}
