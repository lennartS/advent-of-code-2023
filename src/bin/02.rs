// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::cubebag::*;

fn main() -> Result<()> {
    let f = File::open("input-02.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let games = parse(&lines).expect("Could not parse list of games");

    let part1 = CubeSet::new_part1();
    let possible_games = games
        .iter()
        .filter(|g| g.is_possible(&part1))
        .collect::<Vec<_>>();
    let part1_result: i32 = possible_games
        .iter()
        .map(|g| g.id)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("part 1: {:?}", part1_result);

    let min_powers = games
        .iter()
        .map(|g| g.fewest_cubes())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.power())
        .collect::<Vec<_>>();
    let part2_result: i32 = min_powers.iter().sum();
    println!("part 2: {}", part2_result);

    Ok(())
}
