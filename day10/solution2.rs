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
                result += score(x, y, *height, &map);
            }
        }
    }

    println!("{}", result)
}

fn score(x: usize, y: usize, height: usize, map: &Vec<Vec<usize>>) -> usize {
    if map.get(y).is_none() {
        return 0;
    }
    if map[y].get(x).is_none() {
        return 0;
    }
    if height != map[y][x] {
        return 0;
    }
    if height == 9 {
        return 1;
    }
    let mut sum = score(x + 1, y, height + 1, map) + score(x, y + 1, height + 1, map);
    if x != 0 {
        sum += score(x - 1, y, height + 1, map);
    }
    if y != 0 {
        sum += score(x, y - 1, height + 1, map);
    }
    sum
}
