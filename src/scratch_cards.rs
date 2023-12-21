// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-04

use anyhow::{Context, Ok, Result};

#[derive(Debug)]
pub struct Card {
    pub goal: Vec<i32>,
    pub actual: Vec<i32>,
}

impl Card {
    pub fn new(goal: Vec<i32>, actual: Vec<i32>) -> Card {
        Card { goal, actual }
    }

    pub fn matches(&self) -> usize {
        let winning = self
            .actual
            .iter()
            .filter(|a| self.goal.contains(a))
            .collect::<Vec<_>>();
        winning.len()
    }

    pub fn worth(&self) -> i32 {
        if self.matches() >= 1 {
            2_i32.pow(self.matches() as u32 - 1)
        } else {
            0
        }
    }
}

pub fn total(cards: &Vec<Card>) -> i32 {
    let mut card_count = vec![1; cards.len()];

    for (card_idx, card) in cards.iter().enumerate() {
        let old = &card_count[card_idx + 1..card_idx + card.matches() + 1];
        let new = old
            .iter()
            .map(|c| c + card_count[card_idx])
            .collect::<Vec<_>>();
        card_count.splice(card_idx + 1..card_idx + card.matches() + 1, new);
    }

    card_count.iter().sum()
}

pub fn parse(lines: &Vec<String>) -> Result<Vec<Card>> {
    let mut cards: Vec<Card> = vec![];
    for line in lines {
        let numbers = line
            .split(':')
            .last()
            .context("Nothing behind colon")?
            .split('|')
            .collect::<Vec<_>>();
        let goals: Vec<i32> = numbers
            .first()
            .context("No pipe found")?
            .split(' ')
            .filter_map(|g| g.parse::<i32>().ok())
            .collect();
        let actuals: Vec<i32> = numbers
            .last()
            .context("No pipe found")?
            .split(' ')
            .filter_map(|a| a.parse::<i32>().ok())
            .collect();
        cards.push(Card::new(goals, actuals));
    }

    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn sum_web() {
        let cards = parse(&example()).unwrap();
        assert_eq!(6, cards.len());
        assert_eq!(13, cards.iter().map(|c| c.worth()).sum());
        assert_eq!(30, total(&cards));
    }
}
