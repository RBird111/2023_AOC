fn main() {
    let test_data = include_str!("test.txt");

    parse_patterns(test_data).into_iter().for_each(|p| {
        println!("\n[PAT]");
        p.into_iter().for_each(|l| println!("{l:?}"))
    })
}

fn parse_patterns(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|pat| pat.lines().map(|l| l.chars().collect()).collect())
        .collect()
}
