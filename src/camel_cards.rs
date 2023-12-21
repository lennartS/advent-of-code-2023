// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-07

use anyhow::{Ok, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq)]
pub struct Hand {
    pub hand: String,
    pub hand_type: Type,
    pub bid: i32,
    pub strength: i64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            self.strength.cmp(&other.strength)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Hand {
    pub fn new(hand: String, bid: i32) -> Hand {
        let mut cards = HashMap::new();
        for character in hand.chars().collect::<Vec<_>>() {
            let card = cards.entry(character).or_insert(0);
            *card += 1;
        }

        let hand_type = Hand::determine_type_p1(&cards);
        let strength = Hand::determine_strength_p1(&hand);

        Hand {
            hand,
            hand_type,
            bid,
            strength,
        }
    }

    pub fn new_p2(hand: String, bid: i32) -> Hand {
        let mut cards = HashMap::new();
        for character in hand.chars().collect::<Vec<_>>() {
            let card = cards.entry(character).or_insert(0);
            *card += 1;
        }

        let hand_type = Hand::determine_type_p2(&cards);
        let strength = Hand::determine_strength_p2(&hand);

        Hand {
            hand,
            hand_type,
            bid,
            strength,
        }
    }

    fn determine_type_p1(cards: &HashMap<char, i32>) -> Type {
        let values = cards.values().collect::<Vec<_>>();
        if cards.len() == 1 {
            Type::FiveOfAKind
        } else {
            if cards.len() == 2 {
                match values.iter().max() {
                    Some(4) => {
                        return Type::FourOfAKind;
                    }
                    Some(3) => {
                        return Type::FullHouse;
                    }
                    _ => { /* Do nothing */ }
                }
            } else if cards.len() == 3 {
                match values.iter().max() {
                    Some(3) => {
                        return Type::ThreeOfAKind;
                    }
                    Some(2) => {
                        return Type::TwoPair;
                    }
                    _ => { /* Do nothing */ }
                }
            } else if cards.len() == 4 {
                match values.iter().max() {
                    Some(2) => {
                        return Type::OnePair;
                    }
                    Some(1) => {
                        return Type::HighCard;
                    }
                    _ => { /* Do nothing */ }
                }
            }

            Type::HighCard
        }
    }

    fn determine_type_p2(cards: &HashMap<char, i32>) -> Type {
        let nb_jokers = cards.get(&'J').unwrap_or(&0);
        if nb_jokers == &5 {
            return Type::FiveOfAKind;
        }

        let cards_without_jokers = cards
            .iter()
            .filter(|(k, _)| **k != 'J')
            .collect::<HashMap<_, _>>();
        let max_nb_without_jokers = cards_without_jokers
            .values()
            .max()
            .expect("Couldn't get max card");
        let max_card = cards_without_jokers
            .iter()
            .filter(|(_, v)| v == &max_nb_without_jokers)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()[0];

        let mut cards = cards.clone();
        if let Some(x) = cards.get_mut(max_card) {
            *x += nb_jokers;
        }
        cards.remove(&'J');

        Hand::determine_type_p1(&cards)
    }

    fn determine_strength_p1(hand: &str) -> i64 {
        let mut result: i64 = 0;
        for (i, chara) in hand.char_indices() {
            let value: i64 = match chara.to_digit(10) {
                Some(x) => x as i64,
                None => match chara {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => todo!("Unknown card"),
                },
            };
            result += value * 100_i64.pow((6 - i).try_into().expect("error in exponent"));
        }
        result
    }

    fn determine_strength_p2(hand: &str) -> i64 {
        let mut result: i64 = 0;
        for (i, chara) in hand.char_indices() {
            let value: i64 = match chara.to_digit(10) {
                Some(x) => x as i64,
                None => match chara {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => todo!("Unknown card"),
                },
            };
            result += value * 100_i64.pow((6 - i).try_into().expect("error in exponent"));
        }
        result
    }
}

pub fn total_winnings(mut hands: Vec<Hand>) -> i64 {
    let mut result: i64 = 0;
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid as i64 * (i + 1) as i64;
    }
    result
}

pub fn parse(lines: &Vec<String>) -> Result<Vec<Hand>> {
    let mut result = Vec::new();
    for line in lines {
        let token: Vec<&str> = line.split(' ').collect();
        result.push(Hand::new(
            token[0].to_owned(),
            token[1].parse::<i32>().expect("Could not parse bid"),
        ));
    }
    Ok(result)
}

pub fn parse_p2(lines: &Vec<String>) -> Result<Vec<Hand>> {
    let mut result = Vec::new();
    for line in lines {
        let token: Vec<&str> = line.split(' ').collect();
        result.push(Hand::new_p2(
            token[0].to_owned(),
            token[1].parse::<i32>().expect("Could not parse bid"),
        ));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let hands = parse(&example()).unwrap();
        assert_eq!(6440, total_winnings(hands));
    }

    #[test]
    fn result_web_p2() {
        let hands = parse_p2(&example()).unwrap();
        assert_eq!(5905, total_winnings(hands));
    }
}
