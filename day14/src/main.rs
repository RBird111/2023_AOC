#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let test_data = include_str!("test.txt");
    let actual_data = include_str!("input.txt");

    let mut t1 = part_1(test_data);
    println!("[TEST 1]: {t1}");

    let mut p1 = part_1(actual_data);
    println!("[TEST 1]: {p1}");
}

fn part_1(input: &str) -> usize {
    let mut panel = parse_input(input);
    roll_north(&mut panel);
    calculate_load(&panel)
}

fn calculate_load(panel: &Vec<Vec<char>>) -> usize {
    panel
        .iter()
        .enumerate()
        .map(|(i, r)| r.into_iter().filter(|&&c| c == 'O').count() * (panel.len() - i))
        .sum()
}

fn roll_north(panel: &mut Vec<Vec<char>>) {
    for _ in 0..panel.len() {
        for i in 1..panel.len() {
            for j in 0..panel[i].len() {
                if panel[i][j] == 'O' && panel[i - 1][j] == '.' {
                    panel[i][j] = '.';
                    panel[i - 1][j] = 'O';
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
