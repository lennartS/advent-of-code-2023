// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

// 2023-05

use anyhow::{Context, Ok, Result};

#[derive(Clone, Debug)]
pub struct AlmanacInterval {
    pub start: i64,
    pub dest: i64,
    pub length: i64,
}

#[derive(Debug, Clone, Default)]
pub struct AlmanacMap {
    pub ranges: Vec<AlmanacInterval>,
}

impl AlmanacMap {
    pub fn new() -> AlmanacMap {
        AlmanacMap { ranges: Vec::new() }
    }

    pub fn get(&self, src: &i64) -> i64 {
        for i in &self.ranges {
            if src >= &i.start && src <= &(i.start + i.length) {
                return i.dest + (src - i.start);
            }
        }
        *src
    }

    pub fn get_ranges(&self, interval: &[(i64, i64)]) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        let mut remaining = interval.to_owned();
        while !remaining.is_empty() {
            let tmp_interval = remaining.clone();
            // next_intervals will contain all those which didn't have a mapping this time around
            let mut next_intervals = Vec::new();

            for (start, length) in tmp_interval {
                if length == 0 {
                    continue;
                }

                let my_end = start + length - 1;
                let prev_result_len = result.len();
                for i in &self.ranges {
                    let range_end = i.start + i.length - 1;

                    // tmp is fully contained in range
                    if start >= i.start && my_end <= range_end {
                        result.push((i.dest + start - i.start, length));
                        break;
                    }
                    // tmp fully overlaps range
                    if start <= i.start && my_end >= range_end {
                        // translate matched part
                        result.push((i.dest, i.length));
                        next_intervals.push((start, i.start - start));
                        next_intervals.push((range_end + 1, my_end - range_end));
                        break;
                    }
                    // tmp overlaps beginning of range
                    if start < i.start && my_end >= i.start && my_end <= range_end {
                        result.push((i.dest, my_end - i.start + 1));
                        next_intervals.push((start, i.start - start));
                        break;
                    }
                    // tmp overlaps end of range
                    if start > i.start && start <= range_end && my_end >= range_end {
                        result.push((i.dest + start - i.start, range_end - start + 1));
                        next_intervals.push((range_end + 1, my_end - range_end));
                        break;
                    }
                }

                let any_results_added = prev_result_len != result.len();
                if !any_results_added {
                    result.push((start, length));
                }
            }
            remaining = next_intervals;
        }
        result
    }

    pub fn insert(&mut self, start: i64, length: i64, dest: i64) {
        self.ranges.push(AlmanacInterval {
            start,
            dest,
            length,
        });
    }
}

pub fn seeds_as_pairs(seeds: &[i64]) -> Vec<(i64, i64)> {
    let ranges: Vec<_> = seeds.iter().skip(1).step_by(2).copied().collect();
    let seeds: Vec<_> = seeds.iter().step_by(2).copied().collect();

    let mut result = Vec::new();
    for (seed, range) in seeds.iter().zip(ranges.iter()) {
        result.push((*seed, *range));
    }
    result
}

#[allow(clippy::too_many_arguments)]
pub fn seed_to_location(
    seeds: &Vec<i64>,
    seed_to_soil: &AlmanacMap,
    soil_to_fertilizer: &AlmanacMap,
    fertilizer_to_water: &AlmanacMap,
    water_to_light: &AlmanacMap,
    light_to_temperature: &AlmanacMap,
    temperature_to_humidity: &AlmanacMap,
    humidity_to_location: &AlmanacMap,
) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];

    for seed in seeds {
        let soil = seed_to_soil.get(seed);
        let fertilizer = soil_to_fertilizer.get(&soil);
        let water = fertilizer_to_water.get(&fertilizer);
        let light = water_to_light.get(&water);
        let temperature = light_to_temperature.get(&light);
        let humidity = temperature_to_humidity.get(&temperature);
        let location = humidity_to_location.get(&humidity);

        result.push(location);
    }
    result
}

#[allow(clippy::too_many_arguments)]
pub fn seed_ranges_to_location(
    seeds: &Vec<(i64, i64)>,
    seed_to_soil: &AlmanacMap,
    soil_to_fertilizer: &AlmanacMap,
    fertilizer_to_water: &AlmanacMap,
    water_to_light: &AlmanacMap,
    light_to_temperature: &AlmanacMap,
    temperature_to_humidity: &AlmanacMap,
    humidity_to_location: &AlmanacMap,
) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = vec![];

    for seed in seeds {
        let soil = seed_to_soil.get_ranges(&[*seed]);
        let fertilizer = soil_to_fertilizer.get_ranges(&soil);
        let water = fertilizer_to_water.get_ranges(&fertilizer);
        let light = water_to_light.get_ranges(&water);
        let temperature = light_to_temperature.get_ranges(&light);
        let humidity = temperature_to_humidity.get_ranges(&temperature);
        let location = humidity_to_location.get_ranges(&humidity);

        for loc in location {
            result.push(loc);
        }
    }
    result
}

#[allow(clippy::type_complexity)]
pub fn parse(
    lines: &Vec<String>,
) -> Result<(
    Vec<i64>,
    AlmanacMap,
    AlmanacMap,
    AlmanacMap,
    AlmanacMap,
    AlmanacMap,
    AlmanacMap,
    AlmanacMap,
)> {
    let mut seeds = vec![];
    let mut seed_to_soil = AlmanacMap::new();
    let mut soil_to_fertilizer = AlmanacMap::new();
    let mut fertilizer_to_water = AlmanacMap::new();
    let mut water_to_light = AlmanacMap::new();
    let mut light_to_temperature = AlmanacMap::new();
    let mut temperature_to_humidity = AlmanacMap::new();
    let mut humidity_to_location = AlmanacMap::new();

    let mut maps = 18;

    for line in lines {
        if line.starts_with("seeds: ") {
            seeds = line
                .split(':')
                .last()
                .context("Nothing behind colon")?
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().expect("Couldn't parse seed string"))
                .collect();
            continue;
        }
        match line.as_str() {
            "seed-to-soil map:" => {
                maps = 0;
                continue;
            }
            "soil-to-fertilizer map:" => {
                maps = 1;
                continue;
            }
            "fertilizer-to-water map:" => {
                maps = 2;
                continue;
            }
            "water-to-light map:" => {
                maps = 3;
                continue;
            }
            "light-to-temperature map:" => {
                maps = 4;
                continue;
            }
            "temperature-to-humidity map:" => {
                maps = 5;
                continue;
            }
            "humidity-to-location map:" => {
                maps = 6;
                continue;
            }
            "" => {
                continue;
            }
            &_ => { // Do nothing, normal parsing
            }
        }
        let numbers = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().expect("Couldn't parse map string"))
            .collect::<Vec<_>>();
        if maps == 0 {
            seed_to_soil.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 1 {
            soil_to_fertilizer.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 2 {
            fertilizer_to_water.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 3 {
            water_to_light.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 4 {
            light_to_temperature.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 5 {
            temperature_to_humidity.insert(numbers[1], numbers[2], numbers[0]);
        } else if maps == 6 {
            humidity_to_location.insert(numbers[1], numbers[2], numbers[0]);
        }
    }
    let result = (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        // destination source length
        r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn location_web() {
        let (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ) = parse(&example()).unwrap();
        assert_eq!(10, seed_to_soil.get(&10));
        assert_eq!(0, seed_to_soil.get(&0));
        let locations = seed_to_location(
            &seeds,
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        );
        assert_eq!(vec![82, 43, 86, 35], locations);
    }

    #[test]
    fn map_get_ranges() {
        let mut map = AlmanacMap::new();
        map.insert(5, 5, 50);

        assert_eq!(vec![(1, 2)], map.get_ranges(&vec![(1, 2)]));
        assert_eq!(vec![(50, 1), (4, 1)], map.get_ranges(&vec![(4, 2)]));
        assert_eq!(vec![(54, 1), (10, 2)], map.get_ranges(&vec![(9, 3)]));
        assert_eq!(vec![(52, 2)], map.get_ranges(&vec![(7, 2)]));
        assert_eq!(
            vec![(50, 5), (4, 1), (10, 1)],
            map.get_ranges(&vec![(4, 7)])
        );

        let mut map = AlmanacMap::new();
        map.insert(5, 5, 50);
        map.insert(100, 5, 150);
        assert_eq!(vec![(70, 1)], map.get_ranges(&vec![(70, 1)]));
        assert_eq!(vec![(150, 1)], map.get_ranges(&vec![(100, 1)]));
        assert_eq!(
            vec![(4, 1), (150, 1)],
            map.get_ranges(&vec![(4, 1), (100, 1)])
        );
        assert_eq!(
            vec![(4, 1), (54, 1), (150, 1), (10, 1)],
            map.get_ranges(&vec![(4, 1), (9, 2), (100, 1)])
        );

        let mut map = AlmanacMap::new();
        map.insert(1, 1, 10);
        map.insert(2, 1, 100);
        assert_eq!(vec![(100, 1)], map.get_ranges(&vec![(2, 1)]));
        assert_eq!(vec![(10, 1)], map.get_ranges(&vec![(1, 1)]));
        assert_eq!(vec![(10, 1), (100, 1)], map.get_ranges(&vec![(1, 2)]));
    }

    #[test]
    fn location_range_web() {
        let (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ) = parse(&example()).unwrap();

        // First the winning seed
        assert_eq!(vec![(84, 1)], seed_to_soil.get_ranges(&vec![(82, 1)]));

        let seeds = seeds_as_pairs(&seeds);
        let locations = seed_ranges_to_location(
            &seeds,
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        );
        let location_starts = locations.iter().map(|lr| lr.0).collect::<Vec<_>>();
        let min_loc = location_starts.iter().min().unwrap();
        assert_eq!(&46, min_loc);
    }
}
