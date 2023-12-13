// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-01

use anyhow::{Ok, Result};

pub fn preprocess(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in lines {
        let mut result_line = String::new();
        let mut to_skip = 0;
        for (pos, cha) in line.char_indices() {
            if to_skip > 0 {
                // Skip whole match
                to_skip -= 1;

                // Skip only start of match (overlap mode)
                //to_skip = 0;
                continue;
            }

            if cha == 'o'
                && line.get(pos..pos + 3).is_some()
                && line.get(pos..pos + 3).unwrap() == "one"
            {
                result_line += "1";
                to_skip = 2;
                continue;
            }
            if cha == 't'
                && line.get(pos..pos + 3).is_some()
                && line.get(pos..pos + 3).unwrap() == "two"
            {
                result_line += "2";
                to_skip = 2;
                continue;
            }
            if cha == 't'
                && line.get(pos..pos + 5).is_some()
                && line.get(pos..pos + 5).unwrap() == "three"
            {
                result_line += "3";
                to_skip = 4;
                continue;
            }
            if cha == 'f'
                && line.get(pos..pos + 4).is_some()
                && line.get(pos..pos + 4).unwrap() == "four"
            {
                result_line += "4";
                to_skip = 3;
                continue;
            }
            if cha == 'f'
                && line.get(pos..pos + 4).is_some()
                && line.get(pos..pos + 4).unwrap() == "five"
            {
                result_line += "5";
                to_skip = 3;
                continue;
            }
            if cha == 's'
                && line.get(pos..pos + 3).is_some()
                && line.get(pos..pos + 3).unwrap() == "six"
            {
                result_line += "6";
                to_skip = 2;
                continue;
            }
            if cha == 's'
                && line.get(pos..pos + 5).is_some()
                && line.get(pos..pos + 5).unwrap() == "seven"
            {
                result_line += "7";
                to_skip = 4;
                continue;
            }
            if cha == 'e'
                && line.get(pos..pos + 5).is_some()
                && line.get(pos..pos + 5).unwrap() == "eight"
            {
                result_line += "8";
                to_skip = 4;
                continue;
            }
            if cha == 'n'
                && line.get(pos..pos + 4).is_some()
                && line.get(pos..pos + 4).unwrap() == "nine"
            {
                result_line += "9";
                to_skip = 3;
                continue;
            }
            result_line += &cha.to_string();
        }
        result.push(result_line);
    }
    result
}

pub fn count(lines: &Vec<String>) -> Result<u32> {
    let mut numbers: Vec<u32> = vec![];
    for mut line in lines.clone() {
        line.retain(|c| c.is_ascii_digit());
        let first_nb = line.chars().next().unwrap().to_digit(10).unwrap();
        let second_nb = line.chars().last().unwrap().to_digit(10).unwrap();
        numbers.push(first_nb * 10 + second_nb);
    }
    Ok(numbers.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example01() -> Vec<String> {
        r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example01_02() -> Vec<String> {
        r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn count_web() {
        let result = count(&example01());
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn preprocess_simple() {
        assert_eq!(vec!["1"], preprocess(vec!["one".to_string()]));
        assert_eq!(vec!["2"], preprocess(vec!["two".to_string()]));
        assert_eq!(vec!["3"], preprocess(vec!["three".to_string()]));
        assert_eq!(vec!["4"], preprocess(vec!["four".to_string()]));
        assert_eq!(vec!["5"], preprocess(vec!["five".to_string()]));
        assert_eq!(vec!["6"], preprocess(vec!["six".to_string()]));
        assert_eq!(vec!["7"], preprocess(vec!["seven".to_string()]));
        assert_eq!(vec!["8"], preprocess(vec!["eight".to_string()]));
        assert_eq!(vec!["9"], preprocess(vec!["nine".to_string()]));
    }

    #[test]
    fn preprocess_nonsimple() {
        assert_eq!(vec!["112345"], preprocess(vec!["one12345".to_string()]));
        assert_eq!(vec!["6teen"], preprocess(vec!["sixteen".to_string()]));
    }

    #[test]
    fn preprocess_complex() {
        assert_eq!(
            vec!["1ight89"],
            preprocess(vec!["oneighteight9".to_string()])
        );
        assert_eq!(
            vec!["2n8hre8wo"],
            preprocess(vec!["twoneighthreeightwo".to_string()])
        );
    }

    #[test]
    fn count_web2() {
        let processed = &preprocess(example01_02());
        assert_eq!(
            &vec![
                "219",
                "8wo3",
                "abc123xyz",
                "x2ne34",
                "49872",
                "z1ight234",
                "7pqrst6teen"
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
            processed
        );
        let result = count(processed);
        assert_eq!(result.unwrap(), 281);
    }
}
