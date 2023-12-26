use std::{collections::HashMap, fs, path::Path};

// use ahash::{AHashMap, AHashSet};

type Coord = (usize, usize);

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn tile_neighbour_offsets(tile: u8) -> &'static [(isize, isize)] {
    match tile {
        // b'#' =>
        b'.' => &NEIGHBOURS[0..4],
        b'^' => &NEIGHBOURS[0..1],
        b'v' => &NEIGHBOURS[1..2],
        b'<' => &NEIGHBOURS[2..3],
        b'>' => &NEIGHBOURS[3..4],
        _ => unreachable!("neighbours for #"),
    }
}

fn dfs(
    v: Coord,
    goal: &Coord,
    graph: &HashMap<Coord, Vec<(usize, usize, usize)>>,
    seen: &mut Vec<Vec<bool>>,
) -> Option<usize> {
    // DFS-based search where we find the largest path

    // the function returns None if it never found the goal
    // or Some(n) if it did and where n is the distance to the goal

    if v.0 == goal.0 {
        return Some(0);
    }

    let mut max = None;
    for w in graph[&v].iter() {
        // let coord = (w.0, w.1);
        if !seen[w.0][w.1] {
            seen[w.0][w.1] = true;
            let res = dfs((w.0, w.1), goal, graph, seen);
            // if goal is found, save it to max if it's longer
            match res {
                Some(n) => {
                    max = Some(max.unwrap_or(0).max(w.2 + n));
                }
                None => (),
            }
            // remove from seen to allow for more than one path to be found
            seen[w.0][w.1] = false;
        }
    }

    max
}

fn graph_solve(maze: &Vec<&[u8]>, is_p2: bool) -> usize {
    let mut graph = HashMap::<Coord, Vec<(usize, usize, usize)>>::new();

    // build graph from each tile to its neighbours
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            let tile: u8 = maze[y][x];
            if tile == b'#' {
                continue;
            }
            let neighbours = match is_p2 {
                false => tile_neighbour_offsets(tile),
                true => &NEIGHBOURS,
            };

            let vec = graph.entry((y, x)).or_default();

            for &(dy, dx) in neighbours {
                if (y == 0 && dy == -1) || (y == maze.len() - 1 && dy == 1) {
                    continue;
                }
                let ny = (y as isize + dy) as usize;
                let nx = (x as isize + dx) as usize;

                if maze[ny][nx] != b'#' {
                    vec.push((ny, nx, 1))
                }
            }
        }
    }

    // filter to nodes which have two connections and so are a single path
    let corridors = graph
        .iter()
        .filter_map(|(src, dests)| match dests.len() {
            2 => Some(*src),
            _ => None,
        })
        .collect::<Vec<Coord>>();

    for (y, x) in corridors {
        let neighbours = graph.remove(&(y, x)).unwrap();
        let (y1, x1, d1) = neighbours[0];
        let (y2, x2, d2) = neighbours[1];
        let n1 = graph.get_mut(&(y1, x1)).unwrap();

        if let Some(i) = n1.iter().position(|&(r, c, _)| (r, c) == (y, x)) {
            n1[i] = (y2, x2, d1 + d2);
        }

        let n2 = graph.get_mut(&(y2, x2)).unwrap();
        if let Some(i) = n2.iter().position(|&(r, c, _)| (r, c) == (y, x)) {
            n2[i] = (y1, x1, d1 + d2);
        }
    }

    let goal = (maze.len() - 1, maze.len() - 2);
    let mut seen: Vec<Vec<bool>> = vec![vec![false; maze.len()]; maze.len()];

    match dfs((0, 1), &goal, &graph, &mut seen) {
        Some(n) => n,
        None => unreachable!("path not found"),
    }
}

pub fn solve(file_path: &Path) {
    // AOC 2023 Day 23 P1+P2
    // largely based on https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs
    println!("Day 23");
    let data = fs::read_to_string(file_path).expect("Could not read input");

    let maze: Vec<&[u8]> = data.lines().map(|it| it.as_bytes()).collect();
    assert!(maze.len() == maze.first().unwrap().len());

    let res = graph_solve(&maze, false);
    println!("First part results: {res}");

    let res2 = graph_solve(&maze, true);
    println!("Second part results: {res2}\n")
}
