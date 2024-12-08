const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut obstructions = vec![];
    let mut visited = vec![];
    let mut pos = (0, 0);
    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir = 0;
    let mut w = 0;
    let mut h = 0;
    for (y, line) in INPUT.lines().enumerate() {
        h += 1;
        w = line.len() as isize;
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => obstructions.push((x as isize, y as isize)),
                '^' => pos = (x as isize, y as isize),
                _ => (),
            }
        }
    }

    visited.push(pos.clone());

    loop {
        let d = dirs[dir];
        if pos.0 + d.0 < 0 || pos.0 + d.0 == w {
            break;
        };
        if pos.1 + d.1 < 0 || pos.1 + d.1 == h {
            break;
        };
        if obstructions.contains(&(pos.0 + d.0, pos.1 + d.1)) {
            dir += 1;
            dir = dir % 4;
        } else {
            pos = (pos.0 + d.0, pos.1 + d.1);
            if !visited.contains(&pos) {
                visited.push(pos.clone());
            };
        }
    }

    println!("{}", visited.len());
}
