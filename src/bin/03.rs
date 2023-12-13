// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::gear_ratios::*;

fn main() -> Result<()> {
    let f = File::open("input-03.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let schematic = parse(&lines).expect("Could not parse list of games");

    let part_numbers = schematic
        .grow(
            &schematic
                .unite(&schematic.get_neighboring_digits().expect("no neighbors"))
                .expect("no unions"),
        )
        .expect("no growth");
    let part1_result: i32 = part_numbers
        .iter()
        .map(|span| schematic.as_number(span))
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("part 1: {:?}", part1_result);

    let gears = schematic
        .get_gears(&part_numbers)
        .expect("couldn't find gears");
    let geared_part_numbers = part_numbers
        .iter()
        .filter(|pn| gears.contains(&pn.symbol))
        .collect::<Vec<_>>();
    let mult1 = geared_part_numbers.iter().step_by(2).collect::<Vec<_>>();
    let mult2 = geared_part_numbers
        .iter()
        .skip(1)
        .step_by(2)
        .collect::<Vec<_>>();
    let part2_result: i32 = mult1
        .iter()
        .zip(mult2.iter())
        .map(|(a, b)| schematic.as_number(a) * schematic.as_number(b))
        .sum();
    println!("part 2: {}", part2_result);

    Ok(())
}
