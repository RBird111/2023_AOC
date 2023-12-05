#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::HashMap;
use std::ops::Range;

type RangeMap = HashMap<Range<u64>, Box<dyn Fn(u64) -> u64>>;

fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day05/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day05/src/input.txt")?;

    let t1 = part_1(&test);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}\n");

    Ok(())
}

fn part_1(input: &String) -> u64 {
    let groups = get_groups(input);
    let names: Vec<String> = get_names();

    get_seeds(input)
        .into_iter()
        .map(|s| map_recurse(s, names.clone(), &groups))
        .min()
        .unwrap_or(0)
}

fn map_recurse(seed: u64, mut names: Vec<String>, groups: &HashMap<String, RangeMap>) -> u64 {
    let name = match names.pop() {
        Some(s) => s,
        None => return seed,
    };

    let map = &groups[&name];
    let new_seed = match map
        .into_iter()
        .find_map(|(r, f)| r.contains(&seed).then(|| f))
    {
        Some(fun) => fun(seed),
        None => seed,
    };

    map_recurse(new_seed, names, groups)
}

fn get_names() -> Vec<String> {
    vars::GROUPS
        .iter()
        .skip(1)
        .rev()
        .map(|s| s.to_string())
        .collect()
}

fn get_groups(input: &String) -> HashMap<String, RangeMap> {
    let captures = vars::PATTERN.captures(input).unwrap();
    let get_name = |name: &str| captures.name(name).unwrap().as_str().trim();

    vars::GROUPS
        .iter()
        .skip(1)
        .map(|&name| (name, get_name(name)))
        .map(parse_values)
        .collect()
}

fn get_seeds(input: &String) -> Vec<u64> {
    let captures = vars::PATTERN.captures(input).unwrap();
    let get_name = |name: &str| captures.name(name).unwrap().as_str().trim();

    get_name("seeds")
        .lines()
        .flat_map(|s| s.split_whitespace().filter_map(|n| n.parse::<u64>().ok()))
        .collect()
}

fn parse_values((key, value): (&str, &str)) -> (String, RangeMap) {
    (key.to_string(), value.lines().map(make_entry).collect())
}

fn make_entry(s: &str) -> (Range<u64>, Box<dyn Fn(u64) -> u64>) {
    let (t, f, i) = str_to_vals(s);
    (f..f + i, Box::new(move |n| n - f + t))
}

fn str_to_vals(s: &str) -> (u64, u64, u64) {
    let vals: Vec<_> = s
        .split_whitespace()
        .filter_map(|c| c.parse::<u64>().ok())
        .collect();
    (vals[0], vals[1], vals[2])
}

pub mod vars {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref PATTERN: Regex = Regex::new(concat!(
            r"(?m)seeds: (?P<seeds>(\d+\s+)+)\s+",
            r"seed-to-soil map:\s+(?P<soil>(\d+\s+)+)+",
            r"soil-to-fertilizer map:\s+(?P<fertilizer>(\d+\s+)+)+",
            r"fertilizer-to-water map:\s+(?P<water>(\d+\s+)+)+",
            r"water-to-light map:\s+(?P<light>(\d+\s+)+)+",
            r"light-to-temperature map:\s+(?P<temperature>(\d+\s+)+)+",
            r"temperature-to-humidity map:\s+(?P<humidity>(\d+\s+)+)+",
            r"humidity-to-location map:\s+(?P<location>(\d+\s+)+)+",
        ))
        .expect("error parsing regex");
        pub static ref GROUPS: [&'static str; 8] = [
            "seeds",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];
    }
}
