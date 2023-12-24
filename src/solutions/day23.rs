use std::{
    collections::{binary_heap::Iter, BinaryHeap, HashMap, HashSet},
    fmt::Binary,
    fs,
    path::Path,
};

type WeightedCoord = (i32, usize, usize);
type Coord = (usize, usize);

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 23 P1+P2
    println!("Day 23");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    let maze: Vec<&[u8]> = data.lines().map(|it| it.as_bytes()).collect();
    assert!(maze.len() == maze.first().unwrap().len());

    let goal_y = maze.len() - 1;
    let goal_x = maze.len() - 2;

    // hash function?
    // let mut from_map = HashMap::<WeightedCoord, WeightedCoord>::new();
    let mut open = BinaryHeap::<WeightedCoord>::new();
    // binaryheap doesn't have contains()
    let mut open_set = HashSet::<Coord>::new();
    let mut scores = HashMap::<Coord, i32>::new();
    // top left start
    open.push((0, 0, 1));
    open_set.insert((0, 1));
    println!("{open:?}");

    while !open.is_empty() {
        let current = open.pop().unwrap();
        open_set.remove(&(current.1, current.2));
        println!("{current:?}");

        if current.1 + 1 == goal_y && current.2 == goal_x {
            println!("ASDASDSA {current:?}");
            break;
        }

        // if current.0 > -200 {
        //     continue;
        // }

        let tile = maze[current.1][current.2];

        let mut neighbors: Vec<Coord> = vec![];
        match tile {
            b'#' => continue,
            // add the coord the slope brings us to
            b'^' => neighbors.push((current.1 - 1, current.2)),
            b'<' => neighbors.push((current.1, current.2 - 1)),
            b'>' => neighbors.push((current.1, current.2 + 1)),
            b'v' => neighbors.push((current.1 + 1, current.2)),
            b'.' => {
                // add nearby 4
                // we only have to check the coordinate above as
                // in every position that isn't the start there is
                // padding or the tile is the goal
                if current.1 != 0 {
                    neighbors.push((current.1 - 1, current.2));
                }
                neighbors.push((current.1, current.2 - 1));
                neighbors.push((current.1, current.2 + 1));
                neighbors.push((current.1 + 1, current.2));
            }
            _ => unreachable!("malformed input"),
        }

        for neighbor in neighbors {
            let score = current.0 - 1;
            if score < *scores.get(&neighbor).unwrap_or(&9999) {
                scores.insert(neighbor, score);
                if !open_set.contains(&neighbor) {
                    open.push((score, neighbor.0, neighbor.1));
                    open_set.insert(neighbor);
                }
            }
        }
    }
    println!("{:?}", scores.get(&(goal_y, goal_x)).unwrap());

    //println!("First part results: {}", results);
    //println!("Second part results: {results2}\n")
}
