use std::{fs, path::Path};

use regex::Regex;

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 2 P1+P2
    println!("Day 2");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    let legal_reds = 12u32;
    let legal_greens = 13u32;
    let legal_blues = 14u32;

    let mut ids = Vec::<u32>::new();
    // p2
    let mut powers = Vec::<u32>::new();

    let line_re = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let cube_re = Regex::new(r"([0-9]+) (red|blue|green)").unwrap();
    for line in data.lines() {
        // p2 min required cube counts
        let mut min_reds = 0u32;
        let mut min_greens = 0u32;
        let mut min_blues = 0u32;

        let line_caps = line_re
            .captures(line)
            .unwrap_or_else(|| panic!("Could not parse line: {line}"));
        let id = line_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        // 3 blue, 7 green, 10 red; 4 green, 4 red; 1 green, 7 blue, 5 red; 8 blue, 10 red; 7 blue, 19 red, 1 green
        // -> ["3 blue", "7 green", "10 red", "4 red", ...]
        let cubes = line_caps
            .get(2)
            .unwrap()
            .as_str()
            .split(";")
            .flat_map(|it| it.split(","));

        let mut is_correct = true;

        for cube in cubes {
            let cube_caps = cube_re
                .captures(cube)
                .unwrap_or_else(|| panic!("Could not parse line: {line}"));

            let count = cube_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let colour = cube_caps.get(2).unwrap().as_str();

            // check that all cube counts are legal, otherwise flag as incorrect
            match colour {
                "red" => {
                    is_correct = count <= legal_reds;
                    min_reds = min_reds.max(count)
                }
                "green" => {
                    is_correct = count <= legal_greens;
                    min_greens = min_greens.max(count)
                }
                "blue" => {
                    is_correct = count <= legal_blues;
                    min_blues = min_blues.max(count)
                }
                _ => unreachable!(),
            };
        }

        powers.push(min_reds * min_greens * min_blues);

        if is_correct {
            ids.push(id);
        }
    }

    println!("First part answer: {}", ids.iter().sum::<u32>());
    println!("Second part answer: {}\n", powers.iter().sum::<u32>());
}
