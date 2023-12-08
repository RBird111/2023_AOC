#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day08/src/test.txt")?;
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day08/src/input.txt")?;

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> u64 {
    let (route, map) = parse_input(input);
    route
        .chars()
        .cycle()
        .map(|c| c.to_string())
        .scan("AAA".to_string(), |node, step| {
            if *node == "ZZZ" {
                None
            } else {
                *node = map[node][&step].clone();
                Some(1)
            }
        })
        .sum()
}

fn part_2(input: &String) -> u64 {
    let (route, map) = parse_input(input);

    let mut route_iter = route.chars().cycle().map(|c| c.to_string());
    let mut step = || route_iter.next().unwrap();

    let mut nodes: Vec<_> = map
        .keys()
        .filter_map(|s| s.ends_with("A").then(|| (s.clone(), 0)))
        .collect();

    while nodes.iter().any(|(s, _)| !s.ends_with("Z")) {
        let step = step();
        nodes
            .iter_mut()
            .filter(|(s, _)| !s.ends_with("Z"))
            .for_each(|(s, n)| {
                *s = map[s][&step].clone();
                *n += 1;
            });
    }

    nodes
        .into_iter()
        .map(|(_, n)| n)
        .reduce(|m, n| lcm(m, n))
        .unwrap_or(0)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn parse_input(input: &String) -> (String, HashMap<String, HashMap<String, String>>) {
    let (route, nodes) = input
        .split_once("\n")
        .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
        .unwrap();
    (route, parse_nodes(nodes))
}

fn parse_nodes(nodes: String) -> HashMap<String, HashMap<String, String>> {
    let mut map = HashMap::new();

    nodes
        .lines()
        .map(parse_node)
        .for_each(|(val, left, right)| {
            map.insert(
                val,
                HashMap::from([("L".to_string(), left), ("R".to_string(), right)]),
            );
        });

    map
}

fn parse_node(line: &str) -> (String, String, String) {
    let captures = vars::PATTERN.captures(line).expect("error parsing node");

    let get_name = |name: &str| {
        captures
            .name(name)
            .expect("error parsing node value")
            .as_str()
            .to_string()
    };

    (get_name("val"), get_name("left"), get_name("right"))
}

mod vars {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref PATTERN: Regex = Regex::new(
            r"^(?P<val>[A-Z0-9]{3}) = \((?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3})\)$"
        )
        .expect("error parsing regex");
    }
}
