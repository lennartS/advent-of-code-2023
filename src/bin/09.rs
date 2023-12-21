// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::oasis::*;

fn main() -> Result<()> {
    let f = File::open("input-09.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let histories = parse(&lines).unwrap();
    println!(
        "part 1: {}",
        histories.iter().map(|h| h.predict_future()).sum::<i64>()
    );
    println!(
        "part 2: {}",
        histories.iter().map(|h| h.predict_past()).sum::<i64>()
    );

    Ok(())
}
