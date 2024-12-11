use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut map = vec![];
    for line in INPUT.lines() {
        map.push(
            line.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
    }

    let mut result = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                let mut trail_heads = HashSet::new();
                score(x, y, *height, &map, &mut trail_heads);
                result += trail_heads.len();
            }
        }
    }

    println!("{}", result)
}

fn score(
    x: usize,
    y: usize,
    height: usize,
    map: &Vec<Vec<usize>>,
    trail_heads: &mut HashSet<(usize, usize)>,
) {
    if map.get(y).is_none() {
        return;
    }
    if map[y].get(x).is_none() {
        return;
    }
    if height != map[y][x] {
        return;
    }
    if height == 9 {
        trail_heads.insert((x, y));
    }
    score(x + 1, y, height + 1, map, trail_heads);
    score(x, y + 1, height + 1, map, trail_heads);
    if x != 0 {
        score(x - 1, y, height + 1, map, trail_heads);
    }
    if y != 0 {
        score(x, y - 1, height + 1, map, trail_heads);
    }
}
