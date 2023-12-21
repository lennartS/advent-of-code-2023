// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::haunted_wasteland::*;

fn main() -> Result<()> {
    let f = File::open("input-08.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let map = parse(&lines).unwrap();
    println!("part 1: {}", map.moves()?);
    println!("part 2: {}", map.moves_ghost()?);

    Ok(())
}
