// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-02

use anyhow::{Context, Ok, Result};

#[derive(Debug, Clone, Default)]
pub struct CubeSet {
    pub red: Option<i32>,
    pub blue: Option<i32>,
    pub green: Option<i32>,
}

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    draws: Vec<CubeSet>,
}

impl CubeSet {
    pub fn new_part1() -> CubeSet {
        CubeSet {
            red: Some(12),
            blue: Some(14),
            green: Some(13),
        }
    }

    pub fn power(&self) -> i32 {
        self.red.unwrap_or(1) * self.blue.unwrap_or(1) * self.green.unwrap_or(1)
    }
}

impl Game {
    pub fn new(id: i32) -> Game {
        Game { id, draws: vec![] }
    }

    pub fn add_draw(&mut self, draw: CubeSet) {
        self.draws.push(draw);
    }

    pub fn is_possible(&self, cubes: &CubeSet) -> bool {
        let all_reds = self
            .draws
            .iter()
            .map(|set| set.red.unwrap_or(0))
            .collect::<Vec<i32>>();
        let all_blues = self
            .draws
            .iter()
            .map(|set| set.blue.unwrap_or(0))
            .collect::<Vec<i32>>();
        let all_greens = self
            .draws
            .iter()
            .map(|set| set.green.unwrap_or(0))
            .collect::<Vec<i32>>();

        let max_reds = all_reds.iter().max().cloned().unwrap_or(0);
        let max_blues = all_blues.iter().max().cloned().unwrap_or(0);
        let max_greens = all_greens.iter().max().cloned().unwrap_or(0);

        cubes.red.unwrap_or(0) >= max_reds
            && cubes.blue.unwrap_or(0) >= max_blues
            && cubes.green.unwrap_or(0) >= max_greens
    }

    pub fn fewest_cubes(&self) -> CubeSet {
        let all_reds = self
            .draws
            .iter()
            .map(|set| set.red.unwrap_or(0))
            .collect::<Vec<i32>>();
        let max_reds = all_reds.iter().max();
        let all_blues = self
            .draws
            .iter()
            .map(|set| set.blue.unwrap_or(0))
            .collect::<Vec<i32>>();
        let max_blues = all_blues.iter().max();
        let all_greens = self
            .draws
            .iter()
            .map(|set| set.green.unwrap_or(0))
            .collect::<Vec<i32>>();
        let max_greens = all_greens.iter().max();

        CubeSet {
            red: max_reds.copied(),
            blue: max_blues.copied(),
            green: max_greens.copied(),
        }
    }
}

pub fn parse(lines: &Vec<String>) -> Result<Vec<Game>> {
    let mut result = vec![];
    for line in lines {
        let colon = line.split(':');
        let id = colon
            .clone()
            .next()
            .context("prefix before colon not found")?
            .split(' ')
            .last()
            .context("game id not found")?
            .parse::<i32>()?;
        let mut game = Game::new(id);
        for full_draw in colon.last().context("draw not found")?.split(';') {
            let mut set = CubeSet::default();
            for cube in full_draw.split(',') {
                let mut nb_color = cube.trim().split(' ');
                let nb = nb_color.next().context("no number found")?.parse::<i32>()?;
                let color = nb_color.last();
                match color {
                    Some("red") => {
                        set.red = Some(nb);
                    }
                    Some("blue") => {
                        set.blue = Some(nb);
                    }
                    Some("green") => {
                        set.green = Some(nb);
                    }
                    Some(_) => {}
                    None => {}
                }
            }
            game.add_draw(set);
        }
        result.push(game);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn count_web() {
        let example_set = CubeSet::new_part1();
        let games = parse(&example()).unwrap();
        let possible_games = games
            .iter()
            .filter(|g| g.is_possible(&example_set))
            .collect::<Vec<_>>();
        let sum = possible_games
            .iter()
            .map(|g| g.id)
            .collect::<Vec<_>>()
            .iter()
            .sum();
        assert_eq!(8, sum);
    }

    #[test]
    fn fewest_web() {
        let games = parse(&example()).unwrap();
        let min_sets = games.iter().map(|g| g.fewest_cubes()).collect::<Vec<_>>();
        let powers = min_sets.iter().map(|s| s.power()).collect::<Vec<_>>();
        let sum = powers.iter().sum();
        assert_eq!(2286, sum);
    }
}
