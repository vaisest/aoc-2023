use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

pub fn solve(file_path: &Path) {
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
    assert!(
        square.iter().all(|it| it.len() == size),
        "Input is invalid. Input should have equal width and length."
    );

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
                        // part 2, save adjacent asterisk coordinates in set
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
    println!("Second part results: {gear_sum}\n")
}
