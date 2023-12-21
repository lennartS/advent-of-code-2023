// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-10

use anyhow::{bail, Context, Ok, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct LoopNode {
    pub location: (usize, usize),
    pub distance: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Maze {
    pub maze: HashMap<(usize, usize), String>,
    pub start: Option<LoopNode>,
    pub big_loop: Vec<LoopNode>,
    pub loop_hash: HashSet<(usize, usize)>,
    pub start_out: (usize, usize),
    pub start_is: String,
}

impl Maze {
    pub fn add(&mut self, location: (usize, usize), pipe: String) {
        self.maze.insert(location, pipe.clone());
        if pipe == "S" {
            self.start = Some(LoopNode {
                location,
                distance: 0,
            });
        }
    }

    pub fn walk(&mut self, location: LoopNode, from: Option<(usize, usize)>) {
        let symbol = self.maze.get(&location.location).unwrap();
        if let Some(from) = from {
            if symbol.as_str() == "S" {
                self.start_is = if location.location.0 > from.0 {
                    match self.start_out {
                        (0, 1) => "L".to_string(),
                        (1, 0) => "|".to_string(),
                        (0, 5) => "J".to_string(),
                        _ => todo!("error"),
                    }
                } else if location.location.0 < from.0 {
                    match self.start_out {
                        (0, 1) => "F".to_string(),
                        (5, 0) => "|".to_string(),
                        (0, 5) => "7".to_string(),
                        _ => todo!("error"),
                    }
                } else if location.location.1 > from.1 {
                    match self.start_out {
                        (0, 1) => "-".to_string(),
                        (1, 0) => "7".to_string(),
                        (5, 0) => "J".to_string(),
                        _ => todo!("error"),
                    }
                } else if location.location.1 < from.1 {
                    match self.start_out {
                        (1, 0) => "F".to_string(),
                        (5, 0) => "L".to_string(),
                        (0, 5) => "-".to_string(),
                        _ => todo!("error"),
                    }
                } else {
                    "".to_string()
                };
                return;
            }
            let next = match symbol.as_str() {
                "|" => {
                    if from.0 < location.location.0 {
                        LoopNode {
                            location: (location.location.0 + 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0 - 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    }
                }
                "-" => {
                    if from.1 < location.location.1 {
                        LoopNode {
                            location: (location.location.0, location.location.1 + 1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0, location.location.1 - 1),
                            distance: location.distance + 1,
                        }
                    }
                }
                "L" => {
                    if from.0 < location.location.0 {
                        LoopNode {
                            location: (location.location.0, location.location.1 + 1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0 - 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    }
                }
                "J" => {
                    if from.0 < location.location.0 {
                        LoopNode {
                            location: (location.location.0, location.location.1 - 1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0 - 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    }
                }
                "7" => {
                    if from.0 > location.location.0 {
                        LoopNode {
                            location: (location.location.0, location.location.1 - 1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0 + 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    }
                }
                "F" => {
                    if from.0 > location.location.0 {
                        LoopNode {
                            location: (location.location.0, location.location.1 + 1),
                            distance: location.distance + 1,
                        }
                    } else {
                        LoopNode {
                            location: (location.location.0 + 1, location.location.1),
                            distance: location.distance + 1,
                        }
                    }
                }
                _ => todo!("huhu"),
            };
            self.big_loop.push(next.clone());
            self.loop_hash.insert(next.location);
            self.walk(next, Some(location.location));
        } else {
            // Start of maze
            if let Some(right) = self
                .maze
                .get(&(location.location.0, location.location.1 + 1))
            {
                if right == "-" || right == "J" || right == "7" {
                    let next = LoopNode {
                        location: (location.location.0, location.location.1 + 1),
                        distance: 1,
                    };
                    self.big_loop.push(next.clone());
                    self.loop_hash.insert(next.location);
                    self.start_out = (0, 1);
                    self.walk(next, Some(location.location));
                }
            } else if let Some(down) = self
                .maze
                .get(&(location.location.0 + 1, location.location.1))
            {
                if down == "|" || down == "J" || down == "L" {
                    let next = LoopNode {
                        location: (location.location.0 + 1, location.location.1),
                        distance: 1,
                    };
                    self.big_loop.push(next.clone());
                    self.loop_hash.insert(next.location);
                    self.start_out = (1, 0);
                    self.walk(next, Some(location.location));
                }
            }
            // same for left and up
        }
    }

    pub fn furthest_distance(&mut self) -> Result<usize> {
        if let Some(start) = &self.start {
            self.walk(start.clone(), None);
            Ok(self
                .big_loop
                .iter()
                .rev()
                .next()
                .context("No distance found")?
                .distance
                / 2)
        } else {
            bail!("No start found")
        }
    }

    pub fn count_inside(&self, lines: &[String]) -> Result<usize> {
        let mut inside = Vec::new();
        let mut our_maze = self.maze.clone();
        if let Some(start) = our_maze.get_mut(&self.start.clone().unwrap().location) {
            *start = self.start_is.clone();
        }
        // Don't try to be smart
        for (i, line) in lines.iter().enumerate() {
            let mut is_inside = false;
            let mut last_was_l = false;
            let mut last_was_f = false;
            for (j, _) in line.chars().enumerate() {
                let pos = (i, j);
                if self.loop_hash.contains(&pos) {
                    if our_maze.get(&pos).unwrap() == "|" {
                        is_inside = !is_inside;
                    } else if our_maze.get(&pos).unwrap() == "L" {
                        last_was_l = true;
                        last_was_f = false;
                    } else if our_maze.get(&pos).unwrap() == "7" {
                        if last_was_l {
                            is_inside = !is_inside;
                        }
                        last_was_l = false;
                        last_was_f = false;
                    } else if our_maze.get(&pos).unwrap() == "F" {
                        last_was_f = true;
                        last_was_l = false;
                    } else if our_maze.get(&pos).unwrap() == "J" {
                        if last_was_f {
                            is_inside = !is_inside;
                        }
                        last_was_f = false;
                        last_was_l = false;
                    }
                } else if is_inside {
                    inside.push(pos);
                }
            }
        }
        Ok(inside.len())
    }
}

pub fn parse(lines: &[String]) -> Result<Maze> {
    let mut result = Maze::default();
    for (i, line) in lines.iter().enumerate() {
        for (j, column) in line.chars().enumerate() {
            result.add((i, j), column.to_string());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_simple() -> Vec<String> {
        r#"
.....
.S-7.
.|.|.
.L-J.
....."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_dirty() -> Vec<String> {
        r#"
7-F7-
.FJ|7
SJ.L7
|F--J
LJ.LJ"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_p2_small() -> Vec<String> {
        r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_p2_large() -> Vec<String> {
        r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn example_self() -> Vec<String> {
        r#"
.F---7
.||FFJ
.|F-J.
.SJ..."#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[allow(dead_code)]
    fn example_p2_large_junk() -> Vec<String> {
        r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn result_web() {
        let mut maze = parse(&example_simple()).unwrap();
        assert_eq!(4, maze.furthest_distance().unwrap());
        assert_eq!(1, maze.count_inside(&example_simple()).unwrap());
    }

    #[test]
    fn result_web_dirty() {
        let mut maze = parse(&example_dirty()).unwrap();
        assert_eq!(8, maze.furthest_distance().unwrap());
        assert_eq!(1, maze.count_inside(&example_dirty()).unwrap());
    }

    #[test]
    fn result_inner_small() {
        let mut maze = parse(&example_p2_small()).unwrap();
        let _ = maze.furthest_distance();
        assert_eq!(4, maze.count_inside(&example_p2_small()).unwrap());
    }

    #[test]
    fn result_inner_large() {
        let mut maze = parse(&example_p2_large()).unwrap();
        let _ = maze.furthest_distance();
        assert_eq!(8, maze.count_inside(&example_p2_large()).unwrap());
    }

    #[test]
    fn result_inner_self() {
        let mut maze = parse(&example_self()).unwrap();
        let _ = maze.furthest_distance();
        assert_eq!(2, maze.count_inside(&example_self()).unwrap());
    }

    /*
    #[test]
    fn result_inner_junk() {
        let mut maze = parse(&example_p2_large_junk()).unwrap();
        assert!(maze.furthest_distance().is_ok());
        assert_eq!(10, maze.count_inside(&example_p2_large_junk()).unwrap());
    }
    */
}
