fn main() -> std::io::Result<()> {
    let raw_test = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day03/src/test.txt")?;
    let test = parse_input(raw_test);
    test.iter().for_each(|v| {
        v.iter().for_each(|s| print!("{s:^5}"));
        print!("\n");
    });
    println!("");


    let raw_input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day03/src/input.txt")?;
    let input = parse_input(raw_input);

    println!("[TEST 1]: {}", part_1(&test));
    println!("[PART 1]: {}", part_1(&input));

    println!("[TEST 2]: {}", part_2(&test));
    println!("[PART 2]: {}", part_2(&input));

    Ok(())
}

fn part_1(input: &Vec<Vec<String>>) -> i32 {
    (0..input.len())
        .flat_map(|r| {
            (0..input[r].len())
                .filter(|&c| input[r][c].parse::<i32>().is_err() && input[r][c] != ".")
                .flat_map(|c| get_part_numbers(r, c, &input))
                .collect::<Vec<_>>()
        })
        .sum()
}

fn part_2(input: &Vec<Vec<String>>) -> i32 {
    (0..input.len())
        .flat_map(|r| {
            (0..input[r].len())
                .filter(|&c| input[r][c] == "*")
                .map(|c| get_part_numbers(r, c, &input))
                .filter(|v| v.len() == 2)
                .map(|v| v.into_iter().product::<i32>())
                .collect::<Vec<_>>()
        })
        .sum()
}

fn get_part_numbers(r: usize, c: usize, arr: &Vec<Vec<String>>) -> Vec<i32> {
    let (i, j) = (c.saturating_sub(1), (c + 2).min(arr[r].len()));
    let get_line = |arr: &Vec<String>| {
        let mut line: Vec<_> = arr[i..j]
            .into_iter()
            .map(|n| {
                if let Ok(num) = n.parse::<i32>() {
                    num
                } else {
                    0
                }
            })
            .collect();
        line.dedup();
        line.into_iter().filter(|&n| n != 0)
    };

    let mut nums = vec![];

    // Top
    if r > 0 {
        nums.extend(get_line(&arr[r - 1]));
    }

    // Bottom
    if r < arr.len() - 1 {
        nums.extend(get_line(&arr[r + 1]))
    }

    // Left
    if c > 0 {
        if let Ok(num) = arr[r][c - 1].parse::<i32>() {
            nums.push(num)
        }
    }

    // Right
    if c < arr[r].len() - 1 {
        if let Ok(num) = arr[r][c + 1].parse::<i32>() {
            nums.push(num)
        }
    }

    nums
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    let mut arr: Vec<Vec<String>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut r = 0;
    while r < arr.len() {
        let mut c = 0;
        while c < arr[0].len() {
            let mut i = c;
            let mut digits = vec![];
            while let Ok(digit) = arr[r][i].parse::<i32>() {
                digits.push(digit.to_string());
                if i + 1 == arr[0].len() {
                    break;
                }
                i += 1;
            }
            let num: String = digits.concat();
            while c < i {
                arr[r][c] = num.clone();
                c += 1;
            }
            c += 1;
        }
        r += 1;
    }

    arr
}
