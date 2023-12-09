fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day09/src/input.txt")?;

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> i64 {
    parse_input(input)
        .into_iter()
        .map(find_pattern)
        .map(get_next_value)
        .sum()
}

fn part_2(input: &String) -> i64 {
    parse_input(input)
        .into_iter()
        .map(find_pattern)
        .map(get_prev_value)
        .sum()
}

fn get_prev_value(pattern: Vec<Vec<i64>>) -> i64 {
    pattern
        .into_iter()
        .rev()
        .map(|v| v[0])
        .fold(0, |p, n| n - p)
}

fn get_next_value(pattern: Vec<Vec<i64>>) -> i64 {
    pattern.into_iter().rev().map(|v| v[v.len() - 1]).sum()
}

fn find_pattern(history: Vec<i64>) -> Vec<Vec<i64>> {
    (0..)
        .scan(history, |prev, _| {
            if prev.iter().all(|&n| n == 0) {
                None
            } else {
                let mut next = prev.windows(2).map(|arr| arr[1] - arr[0]).collect();
                std::mem::swap(&mut next, prev);
                Some(next)
            }
        })
        .collect()
}

fn parse_input(input: &String) -> Vec<Vec<i64>> {
    input.lines().map(parse_row).collect()
}

fn parse_row(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

#[cfg(test)]
mod test {
    lazy_static::lazy_static! {
        static ref INPUT: String =
            std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day09/src/test.txt")
                .expect("error parsing input");
    }

    #[test]
    fn part_1() {
        let expected = 114;
        let actual = super::part_1(&INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn part_2() {
        let expected = 2;
        let actual = super::part_2(&INPUT);
        assert_eq!(expected, actual)
    }
}
