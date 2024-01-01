// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-11

use anyhow::{Ok, Result};

#[derive(Debug, Clone, Default)]
pub struct Cosmos {
    pub galaxies: Vec<(usize, usize)>,
    pub max_row: usize,
    pub max_col: usize,
}

impl Cosmos {
    pub fn grow(&mut self, age: usize) {
        let used_rows = self.galaxies.iter().map(|g| g.0).collect::<Vec<_>>();
        let used_cols = self.galaxies.iter().map(|g| g.1).collect::<Vec<_>>();

        for row in (0..self.max_row + 1).rev() {
            if !used_rows.contains(&row) {
                for (i, galaxy) in self.galaxies.clone().iter().enumerate() {
                    if galaxy.0 >= row {
                        self.galaxies[i].0 += age;
                    }
                }
            }
        }

        for col in (0..self.max_col + 1).rev() {
            if !used_cols.contains(&col) {
                for (i, galaxy) in self.galaxies.clone().iter().enumerate() {
                    if galaxy.1 >= col {
                        self.galaxies[i].1 += age;
                    }
                }
            }
        }
    }

    pub fn sum_of_shortest_paths(&self) -> u64 {
        let mut result = 0;
        for (i, one) in self.galaxies.iter().enumerate() {
            for other in self.galaxies.iter().skip(i + 1) {
                result += (one.0 as i64 - other.0 as i64).unsigned_abs()
                    + (one.1 as i64 - other.1 as i64).unsigned_abs();
            }
        }
        result
    }
}

pub fn parse(lines: &[String]) -> Result<Cosmos> {
    let mut result = Cosmos::default();
    for (i, line) in lines.iter().enumerate() {
        for (j, column) in line.chars().enumerate() {
            if column == '#' {
                result.galaxies.push((i, j));
            }
        }
        result.max_col = line.len() - 1;
    }
    result.max_row = lines.len() - 1;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let mut cosmos = parse(&example()).unwrap();
        cosmos.grow(1);
        assert_eq!(374, cosmos.sum_of_shortest_paths());
    }

    #[test]
    fn result_web10() {
        let mut cosmos = parse(&example()).unwrap();
        cosmos.grow(9);
        println!("{:?}", cosmos);
        assert_eq!(1030, cosmos.sum_of_shortest_paths());
    }

    #[test]
    fn result_web100() {
        let mut cosmos = parse(&example()).unwrap();
        cosmos.grow(99);
        assert_eq!(8410, cosmos.sum_of_shortest_paths());
    }
}
