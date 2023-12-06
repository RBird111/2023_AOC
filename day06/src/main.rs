#![allow(dead_code, unused_variables)]

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day06/src/input.txt")?;

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> u64 {
    parse_input_1(input)
        .into_iter()
        .map(Race::count_win_scenarios)
        .product()
}

fn part_2(input: &String) -> u64 {
    parse_input_2(input).count_win_scenarios()
}

fn parse_input_1(input: &String) -> Vec<Race> {
    let arr: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .filter_map(|n| n.parse::<u64>().ok())
                .collect()
        })
        .collect();

    std::iter::zip(arr[0].clone(), arr[1].clone())
        .map(Race::new)
        .collect()
}

fn parse_input_2(input: &String) -> Race {
    let arr: Vec<_> = input
        .lines()
        .map(|l| l.split_whitespace().skip(1).collect::<Vec<_>>().concat())
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    Race::new((arr[0], arr[1]))
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new((time, distance): (u64, u64)) -> Self {
        Self { time, distance }
    }

    fn count_win_scenarios(self) -> u64 {
        let get_dist = |t: &u64| (self.time - t) * t;
        let beats_record = |t: &u64| get_dist(t) > self.distance;

        let range: Vec<_> = (0..=self.time).collect();

        let first = range.partition_point(|t| {
            let o = t.saturating_sub(1);
            !beats_record(t) && (get_dist(t) > get_dist(&o))
        });

        let last = range[first..].partition_point(|t| {
            let o = (t + 1).min(self.time);
            beats_record(t) && beats_record(&o)
        }) + first;

        (last + 1).saturating_sub(first) as _
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: String =
            std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day06/src/test.txt")
                .expect("error parsing test.txt");
    }

    #[test]
    fn test_parse_input_1() {
        let expected: Vec<_> = [(7, 9), (15, 40), (30, 200)]
            .into_iter()
            .map(Race::new)
            .collect();
        let actual = parse_input_1(&INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_counts_win_scenarios() {
        let expected = vec![4, 8, 9];
        let actual: Vec<_> = parse_input_1(&INPUT)
            .into_iter()
            .map(Race::count_win_scenarios)
            .collect();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_1() {
        let expected = 288;
        let actual: u64 = part_1(&INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_parse_input_2() {
        let expected = Race::new((71530, 940200));
        let actual = parse_input_2(&INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2() {
        let expected = 71503;
        let actual = part_2(&INPUT);
        assert_eq!(expected, actual)
    }
}
