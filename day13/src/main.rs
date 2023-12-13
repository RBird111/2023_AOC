fn main() {
    let test_data = include_str!("test.txt");
    let actual_data = include_str!("input.txt");

    let t1 = solve(1)(test_data);
    println!("[TEST 1]: {t1}");

    let t2 = solve(2)(test_data);
    println!("[TEST 2]: {t2}");

    let p1 = solve(1)(actual_data);
    println!("[PART 1]: {p1}");

    let p2 = solve(2)(actual_data);
    println!("[PART 2]: {p2}");
}

fn solve(part: usize) -> impl Fn(&str) -> usize {
    move |input| {
        parse_input(input)
            .into_iter()
            .map(get_axis_wrap(part - 1))
            .sum()
    }
}

fn get_axis_wrap(smudge: usize) -> impl Fn(Vec<Vec<char>>) -> usize {
    move |pat: Vec<Vec<char>>| {
        100 * reflect_axis(&pat, smudge) + reflect_axis(&transpose(&pat), smudge)
    }
}

fn reflect_axis(pat: &Vec<Vec<char>>, smudge: usize) -> usize {
    let n = pat.len();
    let is_axis = |&i: &usize| {
        let trim = i.min(n - i);
        let half_1 = pat[..i].into_iter().rev().take(trim).flatten();
        let half_2 = pat[i..].into_iter().take(trim).flatten();
        smudge == half_1.zip(half_2).filter(|(a, b)| a != b).count()
    };
    (1..n).find(is_axis).unwrap_or(0)
}

fn transpose(pat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..pat[0].len())
        .map(|c| pat.into_iter().map(|r| r[c]).collect())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|pat| pat.lines().map(|l| l.chars().collect()).collect())
        .collect()
}
