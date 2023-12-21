// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-03

use anyhow::{Context, Ok, Result};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartNumberSpan {
    pub line: usize,
    pub col_first: usize,
    pub col_last: usize,
    pub symbol: Location,
}

impl PartNumberSpan {
    pub fn new(line: usize, col_first: usize, col_last: usize, symbol: Location) -> PartNumberSpan {
        PartNumberSpan {
            line,
            col_first,
            col_last,
            symbol,
        }
    }
}

#[derive(Debug)]
pub struct Schematic {
    pub lines: Vec<String>,
    pub symbols: Vec<Location>,
}

impl Schematic {
    pub fn get_neighboring_digits(&self) -> Result<Vec<(Location, Location)>> {
        let mut result: Vec<(Location, Location)> = vec![];
        for loc in &self.symbols {
            for line in loc.line - 1..loc.line + 2 {
                for column in loc.column - 1..loc.column + 2 {
                    if let Some(line_window) = self.lines.get(line) {
                        if let Some(cha) = line_window.get(column..column + 1) {
                            if cha.parse::<i32>().is_ok() {
                                result.push((Location { line, column }, loc.clone()));
                            }
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn unite(&self, candidates: &Vec<(Location, Location)>) -> Result<Vec<PartNumberSpan>> {
        let mut result: Vec<PartNumberSpan> = vec![];

        let mut line = candidates
            .first()
            .context("Empty candidates list given")?
            .0
            .line;
        let mut col_first = candidates
            .first()
            .context("Empty candidates list given")?
            .0
            .column;
        let mut col_last = candidates
            .first()
            .context("Empty candidates list given")?
            .0
            .column;
        let mut sym_last = candidates
            .first()
            .context("Empty candidates list given")?
            .1
            .clone();
        for (candidate, neighbor_from) in candidates {
            if candidate.line == line && candidate.column == col_last {
                continue;
            }
            if candidate.line == line && candidate.column == col_last + 1 {
                col_last = candidate.column;
                sym_last = neighbor_from.clone();
            } else {
                result.push(PartNumberSpan {
                    line,
                    col_first,
                    col_last,
                    symbol: sym_last,
                });
                line = candidate.line;
                col_first = candidate.column;
                col_last = candidate.column;
                sym_last = neighbor_from.clone();
            }
        }

        result.push(PartNumberSpan {
            line,
            col_first,
            col_last,
            symbol: candidates
                .last()
                .context("Empty candidates list given")?
                .1
                .clone(),
        });

        Ok(result)
    }

    pub fn grow(&self, spans: &Vec<PartNumberSpan>) -> Result<Vec<PartNumberSpan>> {
        let mut result: Vec<PartNumberSpan> = vec![];

        for current in spans {
            // First grow to the front
            let mut front = current.col_first;
            let mut continue_growing = true;
            while continue_growing {
                if self
                    .lines
                    .get(current.line)
                    .expect("inconsistent spans generated")
                    .get(front..front + 1)
                    .context("Out of bounds during front growing")?
                    .parse::<usize>()
                    .is_ok()
                {
                    match front.checked_sub(1) {
                        Some(x) => {
                            front = x;
                        }
                        None => {
                            break;
                        }
                    }
                } else {
                    front += 1;
                    continue_growing = false;
                }
            }

            // then grow to the back
            let mut back = current.col_last;
            continue_growing = true;
            while continue_growing {
                if self
                    .lines
                    .get(current.line)
                    .expect("inconsistent spans generated")
                    .get(back..back + 1)
                    .context("Out of bounds during back growing")?
                    .parse::<usize>()
                    .is_ok()
                {
                    back += 1;
                    if back
                        >= self
                            .lines
                            .get(current.line)
                            .expect("inconsistent spans generated")
                            .len()
                    {
                        back -= 1;
                        break;
                    }
                } else {
                    back -= 1;
                    continue_growing = false;
                }
            }

            result.push(PartNumberSpan::new(
                current.line,
                front,
                back,
                current.symbol.clone(),
            ));
        }
        Ok(result)
    }

    pub fn as_number(&self, span: &PartNumberSpan) -> i32 {
        self.lines
            .get(span.line)
            .expect("inconsistent span given")
            .get(span.col_first..span.col_last + 1)
            .expect("inconsistent span given")
            .parse::<i32>()
            .expect("invalid span given")
    }

    pub fn get_gears(&self, part_numbers: &[PartNumberSpan]) -> Result<Vec<Location>> {
        let gear_candidates: Vec<Location> = self
            .symbols
            .iter()
            .cloned()
            .filter(|loc| {
                self.lines
                    .get(loc.line)
                    .expect("inconsistent symbols")
                    .get(loc.column..loc.column + 1)
                    .expect("inconsistent symbols")
                    == "*"
            })
            .collect::<Vec<Location>>()
            .to_vec();
        //.iter().cloned().collect::<Vec<_>>();

        let gears = gear_candidates
            .iter()
            .cloned()
            .filter(|cand| {
                part_numbers
                    .iter()
                    .filter(|span| &span.symbol == cand)
                    .collect::<Vec<_>>()
                    .len()
                    == 2
            })
            .collect::<Vec<_>>();

        Ok(gears)
    }
}

pub fn parse(lines: &[String]) -> Result<Schematic> {
    let mut symbs: Vec<Location> = vec![];
    for (line_nb, _) in lines.iter().enumerate() {
        for (idx, cha) in lines[line_nb].char_indices() {
            if !(cha.is_ascii_digit() || cha == '.') {
                symbs.push(Location {
                    line: line_nb,
                    column: idx,
                });
            }
        }
    }

    Ok(Schematic {
        lines: lines.to_owned(),
        symbols: symbs.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_symbols() -> Vec<Location> {
        let mut result = vec![
            Location { line: 1, column: 3 },
            Location { line: 3, column: 6 },
            Location { line: 4, column: 3 },
            Location { line: 5, column: 5 },
            Location { line: 8, column: 3 },
            Location { line: 8, column: 5 },
        ];
        result.sort();
        result
    }

    #[test]
    fn sum_web() {
        let schematic = parse(&example()).unwrap();
        assert_eq!(example_symbols(), schematic.symbols);
        let direct_neighbors: Vec<(Location, Location)> = (|| {
            let mut result = vec![
                (
                    Location { line: 0, column: 2 },
                    Location { line: 1, column: 3 },
                ),
                (
                    Location { line: 2, column: 2 },
                    Location { line: 1, column: 3 },
                ),
                (
                    Location { line: 2, column: 3 },
                    Location { line: 1, column: 3 },
                ),
                (
                    Location { line: 2, column: 6 },
                    Location { line: 3, column: 6 },
                ),
                (
                    Location { line: 2, column: 7 },
                    Location { line: 3, column: 6 },
                ),
                (
                    Location { line: 4, column: 2 },
                    Location { line: 4, column: 3 },
                ),
                (
                    Location { line: 6, column: 4 },
                    Location { line: 5, column: 5 },
                ),
                (
                    Location { line: 7, column: 6 },
                    Location { line: 8, column: 5 },
                ),
                (
                    Location { line: 9, column: 2 },
                    Location { line: 8, column: 3 },
                ),
                (
                    Location { line: 9, column: 3 },
                    Location { line: 8, column: 3 },
                ),
                (
                    Location { line: 9, column: 5 },
                    Location { line: 8, column: 5 },
                ),
                (
                    Location { line: 9, column: 6 },
                    Location { line: 8, column: 5 },
                ),
            ];
            result.sort();
            result
        })();
        assert_eq!(
            direct_neighbors,
            (|| {
                let mut result = schematic.get_neighboring_digits().unwrap();
                result.sort();
                result
            })()
        );

        let united_neighbors: Vec<PartNumberSpan> = (|| {
            let mut result = vec![
                PartNumberSpan::new(0, 2, 2, Location { line: 1, column: 3 }),
                PartNumberSpan::new(2, 2, 3, Location { line: 1, column: 3 }),
                PartNumberSpan::new(2, 6, 7, Location { line: 3, column: 6 }),
                PartNumberSpan::new(4, 2, 2, Location { line: 4, column: 3 }),
                PartNumberSpan::new(6, 4, 4, Location { line: 5, column: 5 }),
                PartNumberSpan::new(7, 6, 6, Location { line: 8, column: 5 }),
                PartNumberSpan::new(9, 2, 3, Location { line: 8, column: 3 }),
                PartNumberSpan::new(9, 5, 6, Location { line: 8, column: 5 }),
            ];
            result.sort();
            result
        })();
        assert_eq!(
            united_neighbors,
            (|| {
                let mut result = schematic.unite(&direct_neighbors).unwrap();
                result.sort();
                result
            })()
        );

        let grown: Vec<PartNumberSpan> = (|| {
            let mut result = vec![
                PartNumberSpan::new(0, 0, 2, Location { line: 1, column: 3 }),
                PartNumberSpan::new(2, 2, 3, Location { line: 1, column: 3 }),
                PartNumberSpan::new(2, 6, 8, Location { line: 3, column: 6 }),
                PartNumberSpan::new(4, 0, 2, Location { line: 4, column: 3 }),
                PartNumberSpan::new(6, 2, 4, Location { line: 5, column: 5 }),
                PartNumberSpan::new(7, 6, 8, Location { line: 8, column: 5 }),
                PartNumberSpan::new(9, 1, 3, Location { line: 8, column: 3 }),
                PartNumberSpan::new(9, 5, 7, Location { line: 8, column: 5 }),
            ];
            result.sort();
            result
        })();
        assert_eq!(
            grown,
            (|| {
                let mut result = schematic.grow(&united_neighbors).unwrap();
                result.sort();
                result
            })()
        );
        assert_eq!(
            4361,
            grown
                .iter()
                .map(|span| schematic.as_number(span))
                .collect::<Vec<_>>()
                .iter()
                .sum()
        );

        let gears = vec![
            Location { line: 1, column: 3 },
            Location { line: 8, column: 5 },
        ];
        assert_eq!(gears, schematic.get_gears(&grown).unwrap());
    }
}
