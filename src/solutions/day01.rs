use std::{fs, path::Path};

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

pub fn solve(file_path: &Path) {
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
