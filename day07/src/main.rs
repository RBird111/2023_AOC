fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day07/src/input.txt")?;

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

static CARD_ORDER_1: &str = "23456789TJQKA";
static CARD_ORDER_2: &str = "J23456789TQKA";

pub fn part_1(input: &String) -> u64 {
    let mut hands: Vec<Vec<u64>> = input.lines().map(|h| parse_hand(h, 1)).collect();
    hands.iter_mut().for_each(score_hand);
    sort_and_rank(&mut hands);
    sum_payout(hands)
}

pub fn part_2(input: &String) -> u64 {
    let mut hands: Vec<Vec<u64>> = input.lines().map(|h| parse_hand(h, 2)).collect();
    hands.iter_mut().for_each(get_best_score);
    sort_and_rank(&mut hands);
    sum_payout(hands)
}

fn parse_hand(s: &str, ordering: u64) -> Vec<u64> {
    use std::collections::HashMap;

    let order = match ordering {
        1 => CARD_ORDER_1,
        2 => CARD_ORDER_2,
        _ => unreachable!(),
    };

    let map: HashMap<u8, u64> = order.bytes().zip(0..).collect();
    let arr: Vec<_> = s.split_ascii_whitespace().collect();
    let hand: Vec<u64> = arr[0].bytes().map(|b| map[&b]).collect();
    let bid: Vec<u64> = vec![arr[1].parse().unwrap()];

    [vec![0], hand, vec![0], bid].concat()
}

fn score_hand(hand: &mut Vec<u64>) {
    let mut count = vec![0; 13];
    hand[1..6]
        .into_iter()
        .map(|c| *c as usize)
        .for_each(|c| count[c] += 1);

    count = count.into_iter().filter(|&n| n != 0).collect();
    count.sort_unstable();

    hand[0] = match &count[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        _ => 0,
    };
}

fn get_best_score(hand: &mut Vec<u64>) {
    let mut possible_hands: Vec<Vec<u64>> = (1..13)
        .map(|b| {
            hand.clone()
                .into_iter()
                .zip(0..)
                .map(|(c, i)| if i > 0 && i < 6 && c == 0 { b } else { c })
                .collect()
        })
        .collect();
    possible_hands.iter_mut().for_each(score_hand);
    possible_hands.sort_unstable_by(sort_hand);
    *hand = possible_hands[0].clone();
}

fn sum_payout(hands: Vec<Vec<u64>>) -> u64 {
    hands.into_iter().map(|h| h[7] * h[6]).sum()
}

fn sort_and_rank(hands: &mut Vec<Vec<u64>>) {
    hands.sort_unstable_by(sort_hand);
    assign_rank(hands);
}

fn sort_hand(a: &Vec<u64>, b: &Vec<u64>) -> std::cmp::Ordering {
    if a[0] == b[0] {
        let mut i = 0;
        while i < 6 && a[i] == b[i] {
            i += 1;
        }
        b[i.min(5)].cmp(&a[i.min(5)])
    } else {
        b.cmp(&a)
    }
}

fn assign_rank(hands: &mut Vec<Vec<u64>>) {
    hands.iter_mut().rev().zip(1..).for_each(|(h, i)| h[6] = i);
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::{part_1, part_2};

    lazy_static! {
        static ref INPUT: String =
            std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day07/src/test.txt")
                .expect("error parsing test.txt");
    }

    #[test]
    fn test_part_1() {
        let expected = 6440;
        let actual = part_1(&INPUT);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2() {
        let expected = 5905;
        let actual = part_2(&INPUT);
        assert_eq!(expected, actual)
    }
}
