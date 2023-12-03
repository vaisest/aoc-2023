use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    time::Instant,
};

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

pub fn three(file_path: &Path) {
    // AOC 2023 Day 3 P1+P2
    println!("Day 3");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    // parse input into a square of bytes since it should be ascii and a square
    let square = data
        .lines()
        .map(|it| {
            if it.is_ascii() {
                it.as_bytes()
            } else {
                panic!("Input is not ASCII")
            }
        })
        .collect::<Vec<&[u8]>>();

    let size = square.get(0).expect("Input is empty").len();
    assert!(square.iter().all(|it| it.len() == size));

    let mut results = Vec::<u32>::new();
    // map from gears to results
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    for j in 0..size {
        let mut number_str = String::new();
        let mut is_number_legal = false;
        // let mut adj_asterisk_coords = Vec::<(usize, usize)>::new();
        let mut adj_asterisk_coords = HashSet::<(usize, usize)>::new();
        for i in 0..size {
            let char = square[j][i];

            if char.is_ascii_digit() {
                // if a digit, save it and try to validate it
                number_str.push(char as char);
            } else if !number_str.is_empty() {
                // if previous was a valid number, add it to results
                if is_number_legal {
                    let num = number_str.parse::<u32>().unwrap();
                    results.push(num);

                    // p2 gears
                    for gear in adj_asterisk_coords.iter() {
                        if !gears.contains_key(&gear) {
                            gears.insert(*gear, vec![num]);
                        } else {
                            gears.get_mut(gear).unwrap().push(num);
                        }
                    }
                }

                // and reset even if not valid
                number_str = String::new();
                is_number_legal = false;
                adj_asterisk_coords.clear();
                continue;
            } else {
                // not a digit and not a p2 gear, ignore
                continue;
            }

            for m in -1..=1i32 {
                for l in -1..=1i32 {
                    // check adjacent indices
                    let y = (j as i32 + m).clamp(0, size as i32 - 1) as usize;
                    let x = (i as i32 + l).clamp(0, size as i32 - 1) as usize;

                    // not same index
                    if y == j as usize && x == i as usize {
                        continue;
                    }

                    let adj_char = square[y][x];
                    // if a special character is adjacent, number is marked ok
                    if !adj_char.is_ascii_digit() && adj_char != '.' as u8 {
                        // part 2
                        if adj_char == '*' as u8 {
                            adj_asterisk_coords.insert((y, x));
                        }
                        is_number_legal = true;
                        break;
                    }
                }
            }
        }

        if is_number_legal {
            let num = number_str.parse::<u32>().unwrap();
            results.push(num);

            for gear in adj_asterisk_coords.iter() {
                if !gears.contains_key(&gear) {
                    gears.insert(*gear, vec![num]);
                } else {
                    gears.get_mut(gear).unwrap().push(num);
                }
            }
        }
    }

    println!("First part results: {}", results.iter().sum::<u32>());
    let gear_sum = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum::<u32>();
    let mut asd: Vec<((usize, usize), &Vec<u32>)> = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|it| (*it.0, it.1))
        .collect();
    asd.sort_by_key(|it| it.0);
    asd.iter().for_each(|it| println!("{it:?}"));
    println!("Second part results: {gear_sum}\n")
}

// pub fn two(file_path: &Path) {
//     // AOC 2023 Day n P1+P2
//     println!("Day n");
//     let data = fs::read_to_string(file_path).expect("Could not read input");

// }
fn main() {
    let start = Instant::now();
    one(Path::new("inputs/input1.txt"));
    two(Path::new("inputs/input2.txt"));
    three(Path::new("inputs/input3.txt"));
    println!("Executed in {:?}", start.elapsed());
}
