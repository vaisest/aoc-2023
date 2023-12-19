use std::{fs, path::Path};

use regex::Regex;

fn win_count(time: f64, distance: f64) -> u32 {
    let sqrt = (time.powi(2) - 4.0 * distance).sqrt();
    let mut left = (time - sqrt) / 2.0;
    let mut right = (time + sqrt) / 2.0;

    // don't count draws
    if left.fract() == 0.0 {
        left += 1.0;
    }
    if right.fract() == 0.0 {
        right -= 1.0;
    }

    // take closest winning integer
    left = left.ceil();
    right = right.floor();

    return 1 + right as u32 - left as u32;
}

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 6 P1+P2
    println!("Day 6");
    let data = fs::read_to_string(file_path).expect("Could not read input");
    let mut lines = data.lines();

    let num_re = Regex::new(r"\s+(\d+)").unwrap();

    // parse races one column at a time, to [(time, distance), ...]
    let race_caps: Vec<(&str, &str)> = num_re
        .captures_iter(lines.next().expect("first line not found"))
        .zip(num_re.captures_iter(lines.next().expect("second line not found")))
        // [(Captures, Captures), ...] -> [(&str, &str), ..]
        .map(|(a, b)| {
            let (_, [time]) = a.extract();
            let (_, [distance]) = b.extract();
            return (time, distance);
        })
        .collect();
    let races = race_caps
        .iter()
        .map(|(a, b)| (a.parse::<f64>().unwrap(), b.parse::<f64>().unwrap()));

    // the distance we travel is equal to the equation tx-x^2,
    //   where t = race time and x is our variable button press
    // so we win when tx-x^2 > d, where d is the distance we are given
    // we can solve x^2-tx+d for x to get the edges for winning:
    let mut result = 1;
    for (time, distance) in races {
        result *= win_count(time, distance);
    }

    // p2, same except race numbers are concatenated
    let (time_concat, distance_concat) = race_caps
        .iter()
        .fold((String::new(), String::new()), |(a_acc, b_acc), (a, b)| {
            (a_acc + a, b_acc + b)
        });
    // note: f32 is not accurate enough for big numbers like in p2
    let big_time = time_concat.parse::<f64>().unwrap();
    let big_distance = distance_concat.parse::<f64>().unwrap();
    let result2 = win_count(big_time, big_distance);

    println!("First part results: {result}");
    println!("Second part results: {result2}\n")
}
