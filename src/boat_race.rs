// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-06

use anyhow::{Ok, Result};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Race {
    pub time: i64,
    pub distance: i64,
}

impl Race {
    pub fn winning_strategies(&self) -> i64 {
        let mut is_winning = Vec::new();
        is_winning.par_extend((1..self.time - 1).into_par_iter().map(|time_pushed| {
            let time_remaining = self.time - time_pushed;
            if (time_remaining * time_pushed) > self.distance {
                1
            } else {
                0
            }
        }));
        is_winning.iter().sum::<i64>()
    }
}

pub fn parse(lines: &[String]) -> Result<Vec<Race>> {
    let times: Vec<i32> = lines[0]
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("Could not parse i32"))
        .collect();
    let distances: Vec<i32> = lines[1]
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().expect("Could not parse i32"))
        .collect();
    let result: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t as i64,
            distance: *d as i64,
        })
        .collect();
    Ok(result)
}

pub fn parse_p2(lines: &[String]) -> Result<Vec<Race>> {
    let times: Vec<i64> = lines[0]
        .replace(' ', "")
        .split(':')
        .skip(1)
        .map(|s| s.parse::<i64>().expect("Could not parse i32"))
        .collect();
    let distances: Vec<i64> = lines[1]
        .replace(' ', "")
        .split(':')
        .skip(1)
        .map(|s| s.parse::<i64>().expect("Could not parse i32"))
        .collect();
    let result: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"Time:      7  15   30
Distance:  9  40  200"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let races = parse(&example()).unwrap();
        assert_eq!(4_i64, races[0].winning_strategies());
        assert_eq!(8_i64, races[1].winning_strategies());
        assert_eq!(9_i64, races[2].winning_strategies());
        assert_eq!(
            288_i64,
            races.iter().map(|r| r.winning_strategies()).product()
        );
    }

    #[test]
    fn result_web2() {
        let races = parse_p2(&example()).unwrap();
        assert_eq!(71503, races[0].winning_strategies());
    }
}
