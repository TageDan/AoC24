use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

const dirs: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let mut obstructions = HashSet::new();
    let mut visited_with_dir: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = 0;
    let mut w = 0;
    let mut h = 0;
    for (y, line) in INPUT.lines().enumerate() {
        h += 1;
        w = line.len() as isize;
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.insert((x as isize, y as isize));
                }
                '^' => pos = (x as isize, y as isize),
                _ => (),
            }
        }
    }

    visited_with_dir.insert(pos, vec![0]);

    let c = walk(
        pos,
        dir,
        visited_with_dir.clone(),
        &obstructions,
        true,
        (w, h),
    );

    println!("{c}");
}

fn walk(
    mut pos: (isize, isize),
    mut dir: usize,
    mut visited_dir: HashMap<(isize, isize), Vec<usize>>,
    obstructions: &HashSet<(isize, isize)>,
    add_obstruction: bool,
    (w, h): (isize, isize),
) -> usize {
    let mut result = 0;
    let mut added_obs = HashSet::new();
    loop {
        let d = dirs[dir];
        if pos.0 + d.0 < 0 || pos.0 + d.0 == w {
            if add_obstruction {
                return result;
            } else {
                return 0;
            }
        };
        if pos.1 + d.1 < 0 || pos.1 + d.1 == h {
            if add_obstruction {
                return result;
            } else {
                return 0;
            }
        };
        if obstructions.contains(&(pos.0 + d.0, pos.1 + d.1)) {
            dir += 1;
            dir = dir % 4;
        } else {
            if add_obstruction {
                if !added_obs.contains(&(pos.0 + d.0, pos.1 + d.1)) {
                    let mut new_obstruction = obstructions.clone();
                    new_obstruction.insert((pos.0 + d.0, pos.1 + d.1));
                    added_obs.insert((pos.0 + d.0, pos.1 + d.1));
                    result += walk(
                        pos,
                        dir,
                        visited_dir.clone(),
                        &new_obstruction,
                        false,
                        (w, h),
                    );
                }
            }
            pos = (pos.0 + d.0, pos.1 + d.1);
            if let Some(list) = visited_dir.get_mut(&pos) {
                if list.contains(&dir) {
                    if !add_obstruction {
                        return 1;
                    }
                } else {
                    list.push(dir)
                }
            } else {
                visited_dir.insert(pos.clone(), vec![dir]);
            }
        }
    }
}
