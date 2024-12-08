use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;
    let mut width = 0;
    let mut height = 0;
    let mut grid = vec![];

    for line in INPUT.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
        width = width.max(line.len());
        height += 1;
    }

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            if grid[y][x] == 'A' {
                match (grid[y - 1][x - 1], grid[y + 1][x + 1]) {
                    ('M', 'S') | ('S', 'M') => match (grid[y + 1][x - 1], grid[y - 1][x + 1]) {
                        ('M', 'S') | ('S', 'M') => result += 1,
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    println!("{result}")
}
