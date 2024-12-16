use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut map = vec![];
    for line in INPUT.lines() {
        let mut row = vec![];
        for c in line.trim().chars() {
            row.push(c);
        }
        map.push(row);
    }

    let mut pos = [0, 0];

    let mut result = 0;

    let mut visited = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited.contains(&(x, y)) {
                let mut walls = Walls {
                    up: HashMap::new(),
                    down: HashMap::new(),
                    left: HashMap::new(),
                    right: HashMap::new(),
                };
                let area = plot_values((x, y), &mut visited, &map, &mut walls);
                result += area * edges(walls);
            }
        }
    }
    println!("{result}")
}

fn edges(mut walls: Walls) -> usize {
    let mut num_walls = 0;
    for l in [
        &mut walls.up,
        &mut walls.down,
        &mut walls.right,
        &mut walls.left,
    ] {
        for row in l.values_mut() {
            let mut last = None;
            row.sort();
            for pos in row {
                if let Some(l) = last {
                    if *pos - l > 1 {
                        num_walls += 1;
                    }
                    last = Some(*pos)
                } else {
                    last = Some(*pos);
                    num_walls += 1;
                }
            }
        }
    }

    num_walls
}

struct Walls {
    right: HashMap<usize, Vec<usize>>,
    left: HashMap<usize, Vec<usize>>,
    down: HashMap<usize, Vec<usize>>,
    up: HashMap<usize, Vec<usize>>,
}

fn plot_values(
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>,
    walls: &mut Walls,
) -> usize {
    let c = map[y][x];

    visited.insert((x, y));
    let mut area = 1;
    if y != 0 && map[y - 1][x] == c {
        if !visited.contains(&(x, y - 1)) {
            let a = plot_values((x, y - 1), visited, map, walls);
            area += a;
        }
    } else {
        walls.up.entry(y).or_insert(vec![]).push(x);
    }
    if x != 0 && map[y][x - 1] == c {
        if !visited.contains(&(x - 1, y)) {
            let a = plot_values((x - 1, y), visited, map, walls);
            area += a;
        }
    } else {
        walls.left.entry(x).or_insert(vec![]).push(y);
    }

    if y < map.len() - 1 && map[y + 1][x] == c {
        if !visited.contains(&(x, y + 1)) {
            let a = plot_values((x, y + 1), visited, map, walls);
            area += a;
        }
    } else {
        walls.down.entry(y).or_insert(vec![]).push(x);
    }
    if x < map[y].len() - 1 && map[y][x + 1] == c {
        if !visited.contains(&(x + 1, y)) {
            let a = plot_values((x + 1, y), visited, map, walls);
            area += a;
        }
    } else {
        walls.right.entry(x).or_insert(vec![]).push(y);
    }

    return area;
}
