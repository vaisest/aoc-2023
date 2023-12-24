use std::{collections::HashMap, fs, path::Path};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get(&self, field: &str) -> u64 {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!("Wrong struct field"),
        }
    }
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn compare_op(part: &Part, field: &str, operator: &str, num: u64) -> bool {
    // println!("{:?}, {:?}, {:?}, {:?}", part, field, operator, num);
    match operator {
        "<" => part.get(field) < num,
        ">" => part.get(field) > num,
        _ => unreachable!("Incorrect operator"),
    }
}

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 19 P1+P2
    println!("Day 19");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    // separated by empty lines
    let mut blocks = data.split("\n\n");

    // parse workflows
    let workflows: HashMap<&str, &str> = blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once("{").expect("invalid input");
            return (name, rest.strip_suffix("}").unwrap());
        })
        .collect();

    let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    // parse parts from the second block
    let parts = blocks.next().unwrap().lines().map(|line| {
        let caps = part_re.captures(line).unwrap();
        Part {
            x: caps[1].parse().unwrap(),
            m: caps[2].parse().unwrap(),
            a: caps[3].parse().unwrap(),
            s: caps[4].parse().unwrap(),
        }
    });

    let mut accepted: Vec<Part> = Vec::new();
    let op_re = Regex::new(r"(x|m|a|s)(<|>)(\d+):(A|R|.+)").unwrap();
    // we process each part until it's either accepted or rejected
    'partfor: for part in parts {
        let mut flow = *workflows.get("in").unwrap();
        loop {
            for flow_cmd in flow.split(",") {
                let mut dest = flow_cmd.to_owned();
                if let Some(caps) = op_re.captures(flow_cmd) {
                    // if the rule in the operation matches, we set
                    // the operation to the destination

                    if compare_op(&part, &caps[1], &caps[2], caps[3].parse().unwrap()) {
                        dest = caps[4].to_string();
                    } else {
                        continue;
                    }
                }

                // dest must be a single goto or A/R by now
                match dest.as_str() {
                    "A" => {
                        accepted.push(part);
                        continue 'partfor;
                    }
                    "R" => continue 'partfor,
                    _ => {
                        flow = *workflows
                            .get(&dest.as_str())
                            .unwrap_or_else(|| panic!("{dest}, {workflows:?}"));
                        break;
                    }
                }
            }
        }
    }

    let result = accepted.iter().map(|it| it.sum()).sum::<u64>();
    println!("First part results: {:?}", result);
    // TODO: P2
    //println!("Second part results: {results2}\n")
}
