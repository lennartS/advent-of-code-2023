// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-09

use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct History {
    pub readings: Vec<i64>,
}

impl History {
    fn recurse(values: &[i64]) -> Vec<i64> {
        if values.iter().any(|&i| i != 0) {
            let mut down = Vec::new();
            for (i, val) in values.iter().enumerate().skip(1) {
                down.push(val - values[i - 1]);
            }
            let result = History::recurse(&down);
            let mut up = values.to_vec();
            up.push(values.last().unwrap() + result.last().unwrap());
            up
        } else {
            let mut result = values.to_vec();
            result.push(0);
            result
        }
    }

    pub fn predict_future(&self) -> i64 {
        let future = History::recurse(&self.readings);
        *future.iter().rev().next().unwrap()
    }

    fn recurse_past(values: &[i64]) -> Vec<i64> {
        if values.iter().any(|&i| i != 0) {
            let mut down = Vec::new();
            for (i, val) in values.iter().enumerate().skip(1) {
                down.push(val - values[i - 1]);
            }
            let result = History::recurse_past(&down);
            let mut up = values.to_vec();
            up.insert(0, values.first().unwrap() - result.first().unwrap());
            up
        } else {
            let mut result = values.to_vec();
            result.insert(0, 0);
            result
        }
    }

    pub fn predict_past(&self) -> i64 {
        let past = History::recurse_past(&self.readings);
        *past.first().unwrap()
    }
}

pub fn parse(lines: &[String]) -> Result<Vec<History>> {
    let mut histories = Vec::new();
    for line in lines {
        let readings = line
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();
        histories.push(History { readings });
    }
    Ok(histories)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let histories = parse(&example()).unwrap();
        assert_eq!(
            114,
            histories.iter().map(|h| h.predict_future()).sum::<i64>()
        );
    }

    #[test]
    fn puzzle_input_sample() {
        let input = vec!["25 50 95 171 295 490 783 1201 1765 2482 3335 4271 5187 5914 6199 5685 3889 178 -6257 -16429 -31585".to_owned()];
        let histories = parse(&input).unwrap();
        assert_eq!(
            -53238,
            histories.iter().map(|h| h.predict_future()).sum::<i64>()
        );
    }

    #[test]
    fn result_web_part2() {
        let input = vec!["10  13  16  21  30  45".to_owned()];
        let histories = parse(&input).unwrap();
        assert_eq!(5, histories.iter().map(|h| h.predict_past()).sum::<i64>());
    }

    #[test]
    fn result_web_part2_all() {
        let histories = parse(&example()).unwrap();
        assert_eq!(2, histories.iter().map(|h| h.predict_past()).sum::<i64>());
    }
}
