use std::{collections::BTreeSet, fs, ops::Range, path::Path};

use regex::Regex;

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

fn map_num(n: u64, map: &Vec<MapRange>) -> u64 {
    for mapping in map.iter() {
        let distance: i64 = n as i64 - mapping.source_start as i64;

        if distance < 0 || distance as u64 >= mapping.len {
            continue;
        }
        return mapping.dest_start + distance as u64;
    }
    return n;
}

fn map_range(range: Range<u64>, map: &Vec<MapRange>) -> Vec<Range<u64>> {
    // based on https://github.com/andypymont/advent2023-rust/blob/main/src/bin/05.rs
    // cuts ranges whenever they intersect with mappings
    let mut cut_positions: BTreeSet<u64> = BTreeSet::new();
    for map in map.iter() {
        let source_end = map.source_start + map.len;

        // range outside mapping
        if range.end < map.source_start || range.start > source_end {
            continue;
        }

        // range overlaps with start
        if map.source_start > range.start {
            cut_positions.insert(map.source_start);
        }

        // range overlaps with end
        if source_end < range.end {
            cut_positions.insert(source_end);
        }
    }
    cut_positions.insert(range.end);

    let mut output = Vec::new();
    let mut current = range.start;

    // we craft new ranges which have been mapped from these cuts
    for position in cut_positions {
        // start is mapped like a normal number
        let start = map_num(current, map);
        // and end is put at the cut position while
        // accounting for how much mapping moved the start value
        let end: u64 = (position as i64 + (start as i64 - current as i64)) as u64;
        output.push(start..end);
        current = position;
    }

    return output;
}

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
    let seeds: Vec<u64> = first_line
        .split_once(": ")
        .expect("incorrect seed list")
        .1
        .split(" ")
        .map(|it| it.parse::<u64>().expect("incorrect seed list"))
        .collect();

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
                dest_start: cap[1].parse::<u64>().unwrap(),
                source_start: cap[2].parse::<u64>().unwrap(),
                len: cap[3].parse::<u64>().unwrap(),
            };

            maps.last_mut().unwrap().push(map);
        }
    }

    let mut smallest = u64::MAX;
    for seed in seeds.iter() {
        // let mut seed_ = *seed;
        // confusing, but maps is a list of maps which are lists of mappings
        smallest = smallest.min(maps.iter().fold(*seed, |acc, map| map_num(acc, map)));
    }

    // p2 seed ranges
    let mut splits: Vec<Range<u64>> = seeds
        .chunks_exact(2)
        // first number is range start, second is range length
        .map(|it| it[0]..(it[0] + it[1]))
        .collect();
    let mut next_round: Vec<Range<u64>> = vec![];

    for map in maps {
        // we calculate stuff by using ranges which are split so that
        // portions of them move based on mappings
        for range in splits.iter() {
            next_round.extend(map_range(range.clone(), &map))
        }
        splits = next_round;
        next_round = Vec::new();
    }

    println!("First part results: {smallest}");
    println!(
        "Second part results: {:?}\n",
        splits.iter().min_by_key(|it| it.start).unwrap().start
    );
}
