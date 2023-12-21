// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::pipe_maze::*;

fn main() -> Result<()> {
    let f = File::open("input-10.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let mut maze = parse(&lines).unwrap();
    println!("part 1: {}", maze.furthest_distance().unwrap());
    println!("part 2: {}", maze.count_inside(&lines).unwrap());

    Ok(())
}
