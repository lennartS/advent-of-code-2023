// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::camel_cards::*;

fn main() -> Result<()> {
    let f = File::open("input-07.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let hands = parse(&lines).expect("Could not parse list of hands");

    let part1_result = total_winnings(hands);
    println!("part 1: {:?}", part1_result);

    let hands_p2 = parse_p2(&lines).expect("Could not parse list of hands for part2");
    let part2_result = total_winnings(hands_p2);
    println!("part 2: {:?}", part2_result);

    Ok(())
}
