// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::scratch_cards::*;

fn main() -> Result<()> {
    let f = File::open("input-04.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let cards = parse(&lines).expect("Could not parse list of cards");

    let part1_result: i32 = cards.iter().map(|c| c.worth()).sum();
    println!("part 1: {:?}", part1_result);

    let part2_result = total(&cards);
    println!("part 2: {}", part2_result);
    Ok(())
}
