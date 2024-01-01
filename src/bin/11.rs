// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::cosmic_expansion::*;

fn main() -> Result<()> {
    let f = File::open("input-11.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut cosmos = parse(&lines).expect("problem parsing");
    cosmos.grow(1);
    println!("part 1: {}", cosmos.sum_of_shortest_paths());
    let mut cosmos = parse(&lines).expect("problem parsing");
    cosmos.grow(999999);
    println!("part 2: {}", cosmos.sum_of_shortest_paths());

    Ok(())
}
