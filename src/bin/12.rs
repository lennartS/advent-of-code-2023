// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::hot_springs::*;

fn main() -> Result<()> {
    let f = File::open("input-12.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut springs = parse(&lines, false).expect("problem parsing");
    println!(
        "part 1: {}",
        springs
            .par_iter_mut()
            .map(|s| s.get_arrangements())
            .sum::<u64>()
    );

    let mut springs = parse(&lines, true).expect("problem parsing");
    println!(
        "part 1: {}",
        springs
            .par_iter_mut()
            .map(|s| s.get_arrangements())
            .sum::<u64>()
    );

    Ok(())
}
