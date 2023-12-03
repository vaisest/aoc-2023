use std::{fs, path::Path, time::Instant};

use regex::Regex;

fn parse_literal_word_digit(x: &str) -> Option<char> {
    // parses e.g. "one" => '1'
    match x {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

pub fn one(file_path: &Path) {
    // AOC 2023 Day 1 P1+P2
    println!("Day 1");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    let mut numbers = Vec::<u32>::new();
    let mut p2_numbers = Vec::<u32>::new();

    for line in data.lines() {
        // first part, find first and last digits
        let mut num_string = String::new();
        let mut last_digit = char::default();

        // second part, match literally written digits with sliding window
        let mut p2_num_string = String::new();
        let mut p2_last_digit = char::default();

        let mut right: usize = 1;
        // check substrings to find out digits from either single digits or literal words
        // could be done twice in first normal and then reverse direction instead
        while right <= line.len() {
            for left in 0..right {
                let slice = &line[left..right];
                // test for single digit
                if slice.len() == 1 {
                    let c = slice.chars().next().unwrap_or_else(|| std::unreachable!());
                    if c.is_ascii_digit() {
                        // part 1
                        if num_string.len() == 0 {
                            num_string.push(c);
                        }
                        last_digit = c;

                        // part 2
                        if p2_num_string.len() == 0 {
                            p2_num_string.push(c);
                        }
                        p2_last_digit = c;
                    }
                }

                let maybe_num = parse_literal_word_digit(slice);
                match maybe_num {
                    Some(c) => {
                        if p2_num_string.len() == 0 {
                            p2_num_string.push(c);
                        }
                        p2_last_digit = c;
                    }
                    None => {}
                }
            }
            right += 1;
        }

        num_string.push(last_digit);
        numbers.push(num_string.parse::<u32>().unwrap_or_else(|_| {
            panic!("Invalid p1 digits parsed. Line: {line}, parsed: {num_string}")
        }));

        p2_num_string.push(p2_last_digit);

        p2_numbers.push(p2_num_string.parse::<u32>().unwrap_or_else(|_| {
            panic!("Invalid p2 digits parsed. Line: {line}, parsed: {p2_num_string}")
        }));
    }

    println!("First part answer: {:?}", numbers.iter().sum::<u32>());
    println!("Second part answer: {:?}\n", p2_numbers.iter().sum::<u32>());
}

pub fn two(file_path: &Path) {
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

// pub fn two(file_path: &Path) {
//     // AOC 2023 Day n P1+P2
//     let data = fs::read_to_string(file_path).expect("Could not read input");

// }
fn main() {
    let start = Instant::now();
    one(Path::new("./input1.txt"));
    two(Path::new("./input2.txt"));
    println!("Executed in {:?}", start.elapsed());
}
