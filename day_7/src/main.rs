use std::collections::HashMap;
use std::fs;

fn main() {
    let puzzle_input: &str = &fs::read_to_string("puzzle_input.txt").unwrap()[..];

    let part_1_sum = part_1(puzzle_input);
    // let part_2_sum = part_2(puzzle_input);

    println!("Part 1: {part_1_sum}");
    // println!("Part 2: {part_2_sum}");
}

fn part_1(puzzle_input: &str) -> i32 {
    let mut game = Game::new();

    let puzzle_lines: Vec<&str> = puzzle_input.split("\n").collect();
    for line in puzzle_lines {
        // println!("{}", line.trim());
        let (hand, bet) = line.trim().split_once(" ").unwrap();

        let cur_play = Play::new(hand.to_string(), bet.parse::<i32>().unwrap());
        // println!("{:?}", cur_play);

        game.plays.push(cur_play);
    }

    game.rank_plays();

    let mut summer: i32 = 0;

    for play in game.plays {
        println!("{:?}, {:?}", play.hand, play.hand_type);
        summer += play.rank * play.bet;
    }

    summer
}

fn part_2(puzzle_input: &str) -> i32 {
    let mut game = Game::new();

    let puzzle_lines: Vec<&str> = puzzle_input.split("\n").collect();
    for line in puzzle_lines {
        println!("{}", line.trim());
        let (hand, bet) = line.trim().split_once(" ").unwrap();

        let cur_play = Play::new(hand.to_string(), bet.parse::<i32>().unwrap());
        // println!("{:?}", cur_play);

        game.plays.push(cur_play);
    }

    game.rank_plays();

    let mut summer: i32 = 0;

    for play in game.plays {
        summer += play.rank * play.bet;
    }

    summer
}

struct Game {
    plays: Vec<Play>,
}

impl Game {
    fn new() -> Self {
        Self { plays: (vec![]) }
    }

    fn rank_plays(&mut self) {
        self.plays.sort_by_key(|s| (s.hand_type, s.hand.clone()));

        self.plays.reverse();

        let mut counter: i32 = 1;

        for play in self.plays.iter_mut() {
            play.rank = counter;
            counter += 1;
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Play {
    hand: Vec<CardType>,
    bet: i32,
    hand_type: HandType,
    rank: i32,
}

impl Play {
    fn new(hand: String, bet: i32) -> Self {
        Self {
            hand: Play::get_cards(&hand),
            bet: bet,
            hand_type: Play::get_hand_type(&hand),
            rank: 0,
        }
    }

    fn get_cards(hand: &str) -> Vec<CardType> {
        let mut card_types: Vec<CardType> = vec![];

        for card in hand.chars().into_iter() {
            match card {
                'A' => card_types.push(CardType::A),
                'K' => card_types.push(CardType::King),
                'Q' => card_types.push(CardType::Queen),
                'T' => card_types.push(CardType::Ten),
                '9' => card_types.push(CardType::Nine),
                '8' => card_types.push(CardType::Eight),
                '7' => card_types.push(CardType::Seven),
                '6' => card_types.push(CardType::Six),
                '5' => card_types.push(CardType::Five),
                '4' => card_types.push(CardType::Four),
                '3' => card_types.push(CardType::Three),
                '2' => card_types.push(CardType::Two),
                'J' => card_types.push(CardType::Jack),
                _ => {}
            }
        }

        return card_types;
    }

    fn get_hand_type(hand: &str) -> HandType {
        let mut card_counter: HashMap<char, i32> = HashMap::new();
        let mut jokers: i32 = 0;

        for card in hand.chars().into_iter() {
            if &card == &'J' {
                jokers += 1;
            } else if card_counter.contains_key(&card) {
                card_counter
                    .entry(card)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            } else {
                card_counter.insert(card, 1);
            }
        }

        let mut values: Vec<&i32> = card_counter.values().into_iter().to_owned().collect();
        values.sort();
        values.reverse();

        let mut add_to_first: i32 = 0;

        if let Some(i) = values.get(0) {
            add_to_first = jokers.clone() + *i;
            values[0] = &add_to_first;
        } else {
            values.push(&jokers);
        }

        match &values[..] {
            [5, ..] => return HandType::FiveOfAKind,
            [4, ..] => return HandType::FourOfAKind,
            [3, 2, ..] => return HandType::FullHouse,
            [3, ..] => return HandType::TreeOfAKind,
            [2, 2, ..] => return HandType::TwoPairs,
            [2, ..] => return HandType::OnePair,
            _ => return HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardType {
    A,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
}

#[cfg(test)]
mod tests {
    // use std::prelude::*;
    use crate::part_1;

    #[test]
    fn part_1_test() {
        let example_puzzle: &str = "32T3K 765 \n        T55J5 684 \n         KK677 28 \n        KTJJT 220 \n        QQQJA 483";

        let res_part_1 = part_1(example_puzzle);
        assert_eq!(res_part_1, 6440);
    }
    #[test]
    fn part_2_test() {
        let example_puzzle: &str = "32T3K 765 \n        T55J5 684 \n         KK677 28 \n        KTJJT 220 \n        QQQJA 483";

        let res_part_2 = part_1(example_puzzle);
        assert_eq!(res_part_2, 5905);
    }
}
