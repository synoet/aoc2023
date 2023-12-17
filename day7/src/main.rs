use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
type Hand = Vec<Card>;

#[derive(Debug)]

struct Bid {
    hand: Hand,
    value: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Card {
    value: i32,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        match s {
            "A" => Card { value: 14 },
            "K" => Card { value: 13 },
            "Q" => Card { value: 12 },
            "J" => Card { value: 11 },
            "T" => Card { value: 10 },
            _ => Card {
                value: s.parse::<i32>().unwrap(),
            },
        }
    }
}

impl From<&str> for Bid {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<&str>>();
        Bid {
            hand: split[0]
                .chars()
                .map(|s| Card::from(s.to_string().as_str()))
                .collect::<Vec<Card>>(),
            value: split[1].parse::<i32>().unwrap(),
        }
    }
}

enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl From<&Vec<Card>> for HandType {
    fn from(hand: &Vec<Card>) -> Self {
        let occurences = hand.get_occurences();
        let hand_type = match occurences.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if occurences.iter().filter(|(count, _)| *count == 4).count() == 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if occurences.iter().filter(|(count, _)| *count == 3).count() == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        };
        hand_type
    }
}

trait GetOccurences {
    fn get_occurences(&self) -> Vec<(usize, i32)>;
}

impl GetOccurences for Vec<Card> {
    fn get_occurences(&self) -> Vec<(usize, i32)> {
        let mut counts = HashMap::new();

        for num in self.iter() {
            *counts.entry(num.value).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .map(|(num, count)| (count, num))
            .collect()
    }
}

impl Bid {
    fn get_hand_value(&self) -> i32 {
        HandType::from(&self.hand) as i32
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_hand_value().cmp(&other.get_hand_value()) {
            Ordering::Equal => {
                for (a, b) in self.hand.iter().zip(other.hand.iter()) {
                    if a.value != b.value {
                        return a.value.cmp(&b.value);
                    }
                }
                Ordering::Equal
            },
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bid {
    fn eq(&self, other: &Self) -> bool {
        self.get_hand_value() == other.get_hand_value()
    }
}

impl Eq for Bid {}

fn part_one() -> i64 {
    let content = read_to_string("input.txt").unwrap();

    let mut bids = content.lines().map(|s| Bid::from(s)).collect::<Vec<Bid>>();

    bids.sort_by(|a, b| a.cmp(b));

    dbg!(&bids.iter().map(|b| b.value).collect::<Vec<i32>>());
    bids.iter().enumerate().map(|(i, b)| (b.value as usize  * (i + 1)) as i64).collect::<Vec<i64>>().iter().sum()
}

fn main() {
    let answer = part_one();

    dbg!(answer);
}
