use itertools::Itertools;
use std::{fs, ops::Range, path::Path};

use regex::Regex;

#[derive(Debug)]
struct MapRange {
    dest_start: u32,
    source_start: u32,
    len: u32,
}

fn map_with(n: &u32, map: &Vec<MapRange>) -> u32 {
    for mapping in map.iter() {
        let distance: i64 = *n as i64 - mapping.source_start as i64;
        if distance >= 0 && distance < mapping.len as i64 {
            return mapping.dest_start + distance as u32;
        }
    }
    return *n;
}

// struct SeedRange {
//     start: u32,
//     end: u32,
// }

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 5 P1+P2
    println!("Day 5");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    // parse the input that's definied in kind of blocks to separate lists
    // so we later iterate over each map
    let mut maps = Vec::<Vec<MapRange>>::new();
    let mut lines = data.lines();

    // first line has seeds on one line
    let first_line = lines.next().expect("no seed list");
    let mut seeds: Vec<u32> = first_line
        .split_once(": ")
        .expect("incorrect seed list")
        .1
        .split(" ")
        .map(|it| it.parse::<u32>().expect("incorrect seed list"))
        .collect();

    // p2 seed ranges
    let p2_seed_ranges: Vec<Range<u32>> = seeds
        .chunks_exact(2)
        // first number is range start, second is range length
        .map(|it| it[0]..(it[0] + it[1]))
        .collect();

    // ranges are merged based on overlapping
    // p2_seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    // let mut p2_seeds: Vec<u32> = p2_seed_ranges.into_iter().flatten().collect();

    // rest of the lines are blocks of the mappings
    // which we save each to a separate list
    let range_re = Regex::new(r"(\d+)\ (\d+)\ (\d+)").unwrap();
    for line in lines {
        // empty lines delimit blocks
        if line == "" {
            maps.push(vec![]);
            continue;
        }
        if let Some(cap) = range_re.captures(line) {
            let map = MapRange {
                dest_start: cap[1].parse::<u32>().unwrap(),
                source_start: cap[2].parse::<u32>().unwrap(),
                len: cap[3].parse::<u32>().unwrap(),
            };

            maps.last_mut().unwrap().push(map);
        }
    }

    for map in maps.iter() {
        for seed in seeds.iter_mut() {
            // confusing, but maps is a list of maps, and map is a list of mappings
            for mapping in map.iter() {
                let distance: i64 = *seed as i64 - mapping.source_start as i64;

                if distance < 0 || distance as u32 >= mapping.len {
                    continue;
                }

                *seed = mapping.dest_start + distance as u32;
                break;
            }
        }
    }

    // same but for p2 seeds
    let mut min = u32::MAX;
    for mut seed in p2_seed_ranges.into_iter().flatten() {
        for map in maps.iter() {
            // confusing, but maps is a list of maps, and map is a list of mappings
            for mapping in map.iter() {
                let distance: i64 = seed as i64 - mapping.source_start as i64;

                if distance < 0 || distance as u32 >= mapping.len {
                    continue;
                }

                seed = mapping.dest_start + distance as u32;
                break;
            }
        }
        min = min.min(seed)
    }

    println!("First part results: {}", seeds.iter().min().unwrap());
    println!("Second part results: {}\n", min)
}
