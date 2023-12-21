// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::boat_race::*;

fn main() -> Result<()> {
    let f = File::open("input-06.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let races = parse(&lines).expect("Could not parse list of races");

    let part1_result: i64 = races.iter().map(|r| r.winning_strategies()).product();
    println!("part 1: {:?}", part1_result);

    let races_p2 = parse_p2(&lines).expect("Could not parse list of races");
    let part2_result: i64 = races_p2.iter().map(|r| r.winning_strategies()).product();
    println!("part 2: {:?}", part2_result);

    Ok(())
}
