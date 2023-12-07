use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", process(input));
}

#[derive(PartialEq, Debug)]
struct Hand {
    bid: i64,
    cards: String,
    hand_type: HandType,
}

#[derive(Debug, PartialEq)]
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
    fn to_val(&self) -> i32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

fn card_to_val(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_val().partial_cmp(&other.to_val())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {
                for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
                    let v1 = card_to_val(c1);
                    let v2 = card_to_val(c2);
                    if v1 > v2 {
                        return Some(std::cmp::Ordering::Greater);
                    }
                    if v1 < v2 {
                        return Some(std::cmp::Ordering::Less);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            ord => ord,
        }
    }
}

fn extract_hand(s: &str) -> Hand {
    let mut parts = s.split(' ');
    let cards = parts.next().unwrap().to_string();
    let bid = parts.next().unwrap().parse().unwrap();

    let mut frequencies: HashMap<char, u32> = HashMap::new();
    for card in cards.chars() {
        *frequencies.entry(card).or_default() += 1;
    }
    let mut list = frequencies.into_iter().map(|c| c.1).collect::<Vec<_>>();
    list.sort();

    let hand_type = match list.as_slice() {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        _ => HandType::HighCard,
    };

    Hand {
        cards,
        bid,
        hand_type,
    }
}

fn process(s: &str) -> i64 {
    let mut hands = s.lines().map(extract_hand).collect::<Vec<_>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .into_iter()
        .zip(1..)
        .map(|(hand, n)| hand.bid * n)
        .sum::<i64>()
}
