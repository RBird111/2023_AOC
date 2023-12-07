use std::cmp::Ordering;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("/home/rburd/code/rust/2023_AOC/day07/src/input.txt")?;

    let p1 = part_1(&input);
    println!("[PART 1]: {p1}");

    let p2 = part_2(&input);
    println!("[PART 2]: {p2}");

    Ok(())
}

fn part_1(input: &String) -> u64 {
    let mut hands: Vec<_> = input.lines().map(Hand::new).collect();
    hands.sort_unstable_by(Hand::compare_cards);
    hands
        .iter_mut()
        .rev()
        .enumerate()
        .for_each(|(idx, hand)| hand.set_rank(idx));
    hands.into_iter().map(|hand| hand.get_payout()).sum()
}

fn part_2(input: &String) -> u64 {
    let mut hands: Vec<_> = input.lines().map(HandAlt::new).collect();
    hands.sort_unstable_by(HandAlt::compare_cards);
    hands
        .iter_mut()
        .rev()
        .enumerate()
        .for_each(|(idx, hand)| hand.set_rank(idx));
    hands
        .into_iter()
        .inspect(|h| println!("{h:?}"))
        .map(|hand| hand.get_payout())
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandAlt {
    hand_type: HandTypeAlt,
    cards: Vec<CardAlt>,
    rank: u64,
    bid: u64,
}

impl HandAlt {
    fn new(line: &str) -> Self {
        let arr: Vec<_> = line.split_ascii_whitespace().collect();
        let (hand, bid) = (arr[0], arr[1]);

        let cards: Vec<CardAlt> = hand.chars().map(CardAlt::new).collect();
        let hand_type = HandTypeAlt::new(&cards);
        let bid: u64 = bid.parse().expect("error parsing bid");

        Self {
            hand_type,
            cards,
            rank: 0,
            bid,
        }
    }

    fn set_rank(&mut self, idx: usize) {
        self.rank = (idx + 1) as u64;
    }

    fn compare_cards(&self, other: &HandAlt) -> Ordering {
        if self.hand_type == other.hand_type {
            let mut i = 0;
            while i < 5 && self.cards[i] == other.cards[i] {
                i += 1;
            }
            other.cards[i.min(4)].cmp(&self.cards[i.min(4)])
        } else {
            self.cmp(&other)
        }
    }

    fn get_payout(&self) -> u64 {
        self.rank * self.bid
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    rank: u64,
    bid: u64,
}

impl Hand {
    fn new(line: &str) -> Self {
        let arr: Vec<_> = line.split_ascii_whitespace().collect();
        let (hand, bid) = (arr[0], arr[1]);

        let cards: Vec<Card> = hand.chars().map(Card::new).collect();
        let hand_type = HandType::new(&cards);
        let bid: u64 = bid.parse().expect("error parsing bid");

        Self {
            hand_type,
            cards,
            rank: 0,
            bid,
        }
    }

    fn set_rank(&mut self, idx: usize) {
        self.rank = (idx + 1) as u64;
    }

    fn compare_cards(&self, other: &Hand) -> Ordering {
        if self.hand_type == other.hand_type {
            let mut i = 0;
            while i < 5 && self.cards[i] == other.cards[i] {
                i += 1;
            }
            other.cards[i.min(4)].cmp(&self.cards[i.min(4)])
        } else {
            self.cmp(&other)
        }
    }

    fn get_payout(&self) -> u64 {
        self.rank * self.bid
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandTypeAlt {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandTypeAlt {
    fn new(cards: &Vec<CardAlt>) -> Self {
        use std::collections::HashMap;

        let arr: Vec<Vec<CardAlt>> = [
            CardAlt::Two,
            CardAlt::Three,
            CardAlt::Four,
            CardAlt::Five,
            CardAlt::Six,
            CardAlt::Seven,
            CardAlt::Eight,
            CardAlt::Nine,
            CardAlt::Ten,
            CardAlt::Queen,
            CardAlt::King,
            CardAlt::Ace,
        ]
        .into_iter()
        .map(|c| {
            cards
                .clone()
                .into_iter()
                .map(|o| if o == CardAlt::Joker { c } else { o })
                .collect()
        })
        .collect();

        let mut hands: Vec<_> = arr
            .into_iter()
            .map(|cards| {
                let mut map: HashMap<CardAlt, u32> = HashMap::new();
                cards.into_iter().for_each(|c| {
                    map.entry(c).and_modify(|v| *v += 1).or_insert(1);
                });

                if map.values().any(|&v| v >= 5) {
                    Self::FiveOfAKind
                } else if map.values().any(|&v| v >= 4) {
                    Self::FourOfAKind
                } else if map.values().any(|&v| v >= 3)
                    && map.values().filter(|&&v| v >= 3 || v >= 2).count() == 2
                {
                    Self::FullHouse
                } else if map.values().any(|&v| v >= 3) {
                    Self::ThreeOfAKind
                } else if map.values().filter(|&&v| v >= 2).count() == 2 {
                    Self::TwoPair
                } else if map.values().any(|&v| v >= 2) {
                    Self::OnePair
                } else {
                    Self::HighCard
                }
            })
            .collect();

        hands.sort_unstable();
        hands[0].clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(cards: &Vec<Card>) -> Self {
        use std::collections::HashMap;

        let mut map: HashMap<Card, u32> = HashMap::new();
        cards.into_iter().for_each(|c| {
            map.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        });

        if map.values().any(|&v| v == 5) {
            Self::FiveOfAKind
        } else if map.values().any(|&v| v == 4) {
            Self::FourOfAKind
        } else if map.values().any(|&v| v == 3)
            && map.values().filter(|&&v| v == 3 || v == 2).count() == 2
        {
            Self::FullHouse
        } else if map.values().any(|&v| v == 3) {
            Self::ThreeOfAKind
        } else if map.values().filter(|&&v| v == 2).count() == 2 {
            Self::TwoPair
        } else if map.values().any(|&v| v == 2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardAlt {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl CardAlt {
    fn new(card: char) -> Self {
        match card {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(card: char) -> Self {
        match card {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
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
