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

    let t2 = part_2(&test);
    println!("[TEST 2]: {t2}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> u64 {
    let groups = get_groups(input);
    let names: Vec<String> = get_names();

    get_seeds(input)
        .into_iter()
        .map(|s| map_recurse(s, names.clone(), &groups))
        .min()
        .unwrap()
}

fn part_2(input: &String) -> u64 {
    let groups = get_groups(input);
    let names: Vec<String> = get_names();
    let seeds: Vec<_> = get_seeds(input)
        .chunks(2)
        .map(|v| v[0]..v[0] + v[1])
        .collect();

    range_recurse(seeds, names, &groups)
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap()
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

fn range_recurse(
    seeds: Vec<Range<u64>>,
    mut names: Vec<String>,
    groups: &HashMap<String, RangeMap>,
) -> Vec<Range<u64>> {
    let name = match names.pop() {
        Some(s) => s,
        None => return seeds,
    };

    let map = &groups[&name];
    let mut new_seeds: Vec<_> = seeds
        .into_iter()
        .flat_map(|s| match map.keys().find(|r| r.contains(&s.start)) {
            Some(r) => split_ranges(&s, r),
            None => split_ranges(&s, &s),
        })
        .collect();

    for _ in 0..10 {
        new_seeds = new_seeds
            .into_iter()
            .flat_map(|s| match map.keys().find(|r| r.contains(&s.start)) {
                Some(r) => split_ranges(&s, r),
                None => split_ranges(&s, &s),
            })
            .collect();
    }

    new_seeds.iter_mut().for_each(|s| {
        if let Some(fun) = map
            .into_iter()
            .find_map(|(r, f)| r.contains(&s.start).then(|| f))
        {
            *s = fun(s.start)..fun(s.end)
        }
    });

    range_recurse(merge_ranges(new_seeds), names, groups)
}

fn split_ranges(this: &Range<u64>, other: &Range<u64>) -> Vec<Range<u64>> {
    let (a, b, x, y) = (this.start, this.end, other.start, other.end);
    if (b < x || a > y) || (a >= x && b <= y) {
        vec![this.clone()]
    } else if a < x && b > y {
        vec![a..x, x..y, y..b]
    } else if a < x && b <= y {
        vec![a..x, x..b]
    } else if a >= x && b > y {
        vec![a..y, y..b]
    } else {
        panic!("unexpected split range case")
    }
}

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_unstable_by_key(|a| a.start);
    let mut merged = Vec::new();
    let mut current_range = ranges[0].clone();
    for range in ranges {
        if range.start <= current_range.end {
            current_range.end = current_range.end.max(range.end);
        } else {
            merged.push(current_range);
            current_range = range;
        }
    }
    merged.push(current_range);
    merged
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
