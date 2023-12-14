#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let test_data = include_str!("test.txt");
    let actual_data = include_str!("input.txt");

    let t1 = part_1(test_data);
    println!("[TEST 1]: {t1}");

    let p1 = part_1(actual_data);
    println!("[PART 1]: {p1}");

    let t2 = part_2(test_data);
    println!("[TEST 2]: {t2}");

    // let p2 = part_2(actual_data);
    // println!("[PART 2]: {p2}");
}

fn part_1(input: &str) -> usize {
    let mut panel = parse_input(input);
    roll_north(&mut panel);
    calculate_total_load(&panel)
}

fn part_2(input: &str) -> usize {
    let mut panel = parse_input(input);

    for i in 0..1000000000 % 7 {
        // let load = calculate_total_load(&panel);
        // println!("[LOAD #{}]: {}", i + 1, load % 10);
        cycle(&mut panel);
    }

    calculate_total_load(&panel)
}

fn calculate_total_load(panel: &Vec<Vec<char>>) -> usize {
    let row_load = |(i, row): (usize, &Vec<char>)| {
        let load = panel.len() - i;
        let stones = row.into_iter().filter(|&&c| c == 'O').count();
        load * stones
    };

    panel.iter().enumerate().map(row_load).sum()
}

fn cycle(panel: &mut Vec<Vec<char>>) {
    roll_north(panel);
    roll_west(panel);
    roll_south(panel);
    roll_east(panel);
}

fn roll_north(panel: &mut Vec<Vec<char>>) {
    let (m, n) = (panel.len(), panel[0].len());
    (1..m).for_each(|i| {
        (0..n).for_each(|j| {
            if panel[i][j] != 'O' {
                return;
            }

            let mut r = i;
            while r > 0 && panel[r - 1][j] == '.' {
                panel[r][j] = '.';
                panel[r - 1][j] = 'O';
                r -= 1;
            }
        })
    });
}

fn roll_south(panel: &mut Vec<Vec<char>>) {
    let (m, n) = (panel.len(), panel[0].len());
    (0..m - 1).rev().for_each(|i| {
        (0..n).for_each(|j| {
            if panel[i][j] != 'O' {
                return;
            }

            let mut r = i;
            while r < m - 1 && panel[r + 1][j] == '.' {
                panel[r][j] = '.';
                panel[r + 1][j] = 'O';
                r += 1;
            }
        })
    });
}

fn roll_east(panel: &mut Vec<Vec<char>>) {
    let (m, n) = (panel.len(), panel[0].len());
    (0..m).rev().for_each(|i| {
        (0..n - 1).rev().for_each(|j| {
            if panel[i][j] != 'O' {
                return;
            }

            let mut r = j;
            while r < n - 1 && panel[i][r + 1] == '.' {
                panel[i][r] = '.';
                panel[i][r + 1] = 'O';
                r += 1;
            }
        })
    });
}

fn roll_west(panel: &mut Vec<Vec<char>>) {
    let (m, n) = (panel.len(), panel[0].len());
    (0..m).rev().for_each(|i| {
        (1..n).for_each(|j| {
            if panel[i][j] != 'O' {
                return;
            }

            let mut r = j;
            while r > 0 && panel[i][r - 1] == '.' {
                panel[i][r] = '.';
                panel[i][r - 1] = 'O';
                r -= 1;
            }
        })
    });
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
