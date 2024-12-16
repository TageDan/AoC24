use std::collections::HashSet;

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
                let (area, perimiter) = plot_values((x, y), &mut visited, &map);
                result += area * perimiter;
            }
        }
    }
    println!("{result}")
}

fn plot_values(
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> (usize, usize) {
    let c = map[y][x];

    visited.insert((x, y));
    let mut perim = 0;
    let mut area = 1;
    if y != 0 && map[y - 1][x] == c {
        if !visited.contains(&(x, y - 1)) {
            let (a, p) = plot_values((x, y - 1), visited, map);
            perim += p;
            area += a;
        }
    } else {
        perim += 1;
    }
    if x != 0 && map[y][x - 1] == c {
        if !visited.contains(&(x - 1, y)) {
            let (a, p) = plot_values((x - 1, y), visited, map);
            perim += p;
            area += a;
        }
    } else {
        perim += 1;
    }

    if y < map.len() - 1 && map[y + 1][x] == c {
        if !visited.contains(&(x, y + 1)) {
            let (a, p) = plot_values((x, y + 1), visited, map);
            perim += p;
            area += a;
        }
    } else {
        perim += 1;
    }
    if x < map[y].len() - 1 && map[y][x + 1] == c {
        if !visited.contains(&(x + 1, y)) {
            let (a, p) = plot_values((x + 1, y), visited, map);
            perim += p;
            area += a;
        }
    } else {
        perim += 1;
    }

    return (area, perim);
}
