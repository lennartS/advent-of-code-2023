// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-08

use anyhow::{Context, Ok, Result};
use num::Integer;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Map {
    pub directions: String,
    pub network: HashMap<String, (String, String)>,
}

impl Map {
    pub fn moves(&self) -> Result<u32> {
        let mut moves = 0;
        let mut location = "AAA";
        for current_move in self.directions.chars().cycle() {
            let node = self
                .network
                .get(location)
                .context("location {location} not found")?;
            if current_move == 'L' {
                location = &node.0;
            } else {
                location = &node.1;
            }
            moves += 1;

            if location == "ZZZ" {
                break;
            }
        }
        Ok(moves)
    }

    pub fn moves_ghost_bruteforce(&self) -> Result<u32> {
        let mut moves = 0;
        let mut location: Vec<_> = self.network.keys().filter(|k| k.ends_with('A')).collect();
        for current_move in self.directions.chars().cycle() {
            let working_location = location.clone();
            location.clear();
            location.par_extend(working_location.par_iter().map(|current_location| {
                let node = self
                    .network
                    .get(current_location.as_str())
                    .expect("location {location} not found");
                if current_move == 'L' {
                    &node.0
                } else {
                    &node.1
                }
            }));
            moves += 1;

            if location
                .par_iter()
                .filter(|l| !l.ends_with('Z'))
                .collect::<Vec<_>>()
                .is_empty()
            {
                break;
            }
        }
        Ok(moves)
    }

    pub fn moves_ghost(&self) -> Result<u64> {
        let mut moves = 0;
        let mut all_moves: Vec<u64> = Vec::new();
        let location: Vec<_> = self.network.keys().filter(|k| k.ends_with('A')).collect();
        for current_location in location {
            let mut tmp_location = current_location;
            for current_move in self.directions.chars().cycle() {
                let node = self
                    .network
                    .get(tmp_location)
                    .expect("location {location} not found");
                if current_move == 'L' {
                    tmp_location = &node.0;
                } else {
                    tmp_location = &node.1;
                }
                moves += 1;

                if tmp_location.ends_with('Z') {
                    break;
                }
            }
            all_moves.push(moves);
            moves = 0;
        }
        let moves = all_moves
            .into_iter()
            .reduce(|a, b| a.lcm(&b))
            .context("Couldn't compute lcm")?;
        Ok(moves)
    }
}

pub fn parse(lines: &[String]) -> Result<Map> {
    let directions = lines.get(0).context("no directions given")?;
    let re = Regex::new(r"(?<from>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)")
        .context("Could not compile regex")?;
    let mut network = HashMap::new();

    for line in lines.iter().filter(|s| !s.is_empty()).skip(1) {
        let Some(captures) = re.captures(line) else {
            panic!("Line {line} did not match regex");
        };

        network.insert(
            captures["from"].to_string(),
            (captures["left"].to_string(), captures["right"].to_string()),
        );
    }
    Ok(Map {
        directions: directions.to_string(),
        network,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example2() -> Vec<String> {
        r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_part2() -> Vec<String> {
        r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let map = parse(&example()).unwrap();
        assert_eq!(2, map.moves().unwrap());
    }

    #[test]
    fn result_web2() {
        let map = parse(&example2()).unwrap();
        assert_eq!(6, map.moves().unwrap());
    }

    #[test]
    fn result_part2() {
        let map = parse(&example_part2()).unwrap();
        assert_eq!(6, map.moves_ghost().unwrap());
    }
}
