// Copyright (C) 2023 Lennart Sauerbeck <devel at lennart dot sauerbeck dot org>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use advent_of_code_2023::almanac::*;

fn main() -> Result<()> {
    let f = File::open("input-05.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse(&lines).unwrap();
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
    let lowest_loc = locations.iter().min();
    println!("part 1: {:?}", lowest_loc);

    // The way of the brute force
    /*
    let seeds_p2 = seeds_as_pairs(&seeds);
    println!("got {} seeds", seeds_p2.len());
    seeds_p2.par_iter().for_each(|(start, range)| {
        println!("{} {}", start, range);
        let loc = seed_to_location(
            &(*start..start + range).collect::<Vec<_>>(),
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        );
        let lowest = loc.iter().min().unwrap();
        println!("{}", lowest);
    });
    println!("part 2: {:?}", lowest_loc);
    */

    // The way of ranges
    let seeds_p2 = seeds_as_pairs(&seeds);
    let locations = seed_ranges_to_location(
        &seeds_p2,
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temperature,
        &temperature_to_humidity,
        &humidity_to_location,
    );
    let starts = locations.par_iter().map(|l| l.0).collect::<Vec<_>>();
    println!("{}", starts.len());
    let lowest = starts.par_iter().min().unwrap();
    println!("part 2: {:?}", lowest);

    Ok(())
}
