use std::{path::Path, time::Instant};
mod solutions;

fn main() {
    let start = Instant::now();
    solutions::day01::solve(Path::new("inputs/input01.txt"));
    solutions::day02::solve(Path::new("inputs/input02.txt"));
    solutions::day03::solve(Path::new("inputs/input03.txt"));
    solutions::day04::solve(Path::new("inputs/input04.txt"));
    solutions::day05::solve(Path::new("inputs/input05.txt"));
    solutions::day06::solve(Path::new("inputs/input06.txt"));
    solutions::day19::solve(Path::new("inputs/input19.txt"));
    solutions::day23::solve(Path::new("inputs/input23.txt"));
    println!("Executed in {:?}", start.elapsed());
}
