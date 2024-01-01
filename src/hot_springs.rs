// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-12

use anyhow::{Ok, Result};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Spring {
    pub original: String,
    step: usize,
    check: Regex,
    nb_check: usize,
    placeholders: Vec<usize>,
}

impl Iterator for Spring {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;

        if self.step - 1 < 2_usize.pow(self.placeholders.len().try_into().unwrap()) {
            let mut result = String::new();
            let mut step_bin = format!("{0:b}", self.step);
            for c in self.original.chars().rev() {
                match c {
                    '?' => {
                        result.insert(0, step_bin.pop().unwrap_or('0'));
                    }
                    _ => {
                        result.insert(0, c);
                    }
                }
            }
            let result = result.replace('0', ".");
            let result = result.replace('1', "#");

                Some(result)
        } else {
            None
        }
    }
}

impl Spring {
    pub fn new(line: String, checkstring: String, unfold: bool) -> Spring {
        let mut checkstring = checkstring;
        if unfold {
            checkstring = format!(
                "{},{},{},{},{}",
                checkstring, checkstring, checkstring, checkstring, checkstring
            );
        }

        let mut line = line;
        if unfold {
            line = format!("{}?{}?{}?{}?{}", line, line, line, line, line);
        }

        let mut regex_str = r"^\.*".to_owned();
        let mut nb_check = 0;
        for number in checkstring.split(',') {
            regex_str.push_str(&format!(r"#{{{}}}\.+", number.parse::<i32>().unwrap()));
            nb_check += number.parse::<usize>().unwrap();
        }
        regex_str.pop();
        regex_str.push_str("*$");
        let check = Regex::new(&regex_str).unwrap();

        Spring {
            step: 0,
            check,
            placeholders: line
                .char_indices()
                .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
                .collect::<Vec<_>>(),
            original: line,
            nb_check,
        }
    }

    pub fn get_arrangements(&mut self) -> u64 {
        let mut result = 0;
        let check = self.check.clone();
        let nb_check = self.nb_check;
        for i in self {
            let nb_damaged = i.chars().filter(|c| *c == '#').collect::<Vec<_>>().len();
                if nb_damaged == nb_check && check.is_match(&i) {
                    result += 1;
                }
        }
        println!("{}", result);
        result
    }
}

pub fn parse(lines: &[String], unfold: bool) -> Result<Vec<Spring>> {
    let mut result = Vec::new();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();
        result.push(Spring::new(
            parts[0].to_string(),
            parts[1].to_string(),
            unfold,
        ));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let mut springs = parse(&example(), false).unwrap();
        assert_eq!(
            vec![1, 4, 1, 1, 4, 10],
            springs
                .iter_mut()
                .map(|s| s.get_arrangements())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn result_web_one() {
        let mut springs = parse(&vec!["???.### 1,1,3".to_owned()], false).unwrap();
        assert_eq!(1_u64, springs[0].get_arrangements());
    }

    #[test]
    fn result_inputs() {
        let mut springs = parse(&vec!["?????????? 1,1,4".to_owned()], false).unwrap(); // 29
        assert_eq!(10_u64, springs[0].get_arrangements());

        let mut springs = parse(&vec!["????.?#.?????????? 4,1,1,1,1,1".to_owned()], false).unwrap(); // 238
        assert_eq!(35_u64, springs[0].get_arrangements());

        let mut springs = parse(&vec!["..????.??..?? 1,2,1,1".to_owned()], false).unwrap(); // 991
        assert_eq!(4_u64, springs[0].get_arrangements());

        let mut springs = parse(&vec!["?#????##??##???.#??# 4,2,6,1,1".to_owned()], false).unwrap(); // 997
        assert_eq!(2_u64, springs[0].get_arrangements());

        let mut springs = parse(&vec!["?#???#?.#???? 6,1,1".to_owned()], false).unwrap(); // 1000
        assert_eq!(6_u64, springs[0].get_arrangements());
    }

    #[test]
    fn unfold_small() {
        let mut springs = parse(&vec!["???.### 1,1,3".to_owned()], true).unwrap();
        assert_eq!(1_u64, springs[0].get_arrangements());
    }

    /*
    #[test]
    fn unfold_example() {
        let mut springs = parse(&example(), true).unwrap();
        assert_eq!(
            vec![1, 16384, 1, 16, 2500, 506250],
            springs
                .iter_mut()
                .map(|s| s.get_arrangements())
                .collect::<Vec<_>>()
        );
    }
    */
}
