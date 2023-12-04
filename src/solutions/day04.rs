use std::{collections::HashSet, fs, path::Path};

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 4 P1+P2
    println!("Day 4");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    // list of card number parts
    let mut cards: Vec<std::str::Split<'_, &str>> = data
        .lines()
        .map(|it| it.split(":").nth(1).expect("Incorrect input").trim())
        .map(|it| it.split("|"))
        .collect();

    let mut sum = 0u32;
    // p2
    let mut copy_count = 0u32;

    // tracks copy count for each card for p2
    let mut copies: Vec<u32> = Vec::with_capacity(cards.len());
    copies.resize(cards.len(), 0);

    for (i, card) in cards.iter_mut().enumerate() {
        // parse winning numbers from splits
        let winners: HashSet<u8> = card
            .next()
            .expect("Incorrect input")
            .split_whitespace()
            .map(|it| it.parse::<u8>().expect("Incorrect input"))
            .collect();

        // count matching numbers from second part of the split
        let match_count: usize = card
            .next()
            .expect("Incorrect input")
            .split_whitespace()
            .map(|it| it.parse::<u8>().expect("Incorrect input"))
            .filter(|it| winners.contains(it))
            .count();

        // p2
        // how many we have of this card in total
        let repeat_count = 1 + *copies.get(i).unwrap();
        copy_count += repeat_count;

        // add copies to the next n copies m times,
        //   where n is how many matches this card got
        //   and   m is how many of this card we have
        for j in i + 1..=i + match_count {
            copies[j] += repeat_count;
        }

        sum += if match_count > 0 {
            2u32.pow(match_count as u32 - 1)
        } else {
            0
        };
    }

    println!("First part results: {sum}");
    println!("Second part results: {copy_count}\n")
}
