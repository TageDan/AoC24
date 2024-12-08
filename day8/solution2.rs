use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut anti_nodes = HashSet::new();
    let mut antenna_locations = HashMap::new();
    let h = INPUT.lines().count() as isize;
    let mut w = 0;

    for (y, line) in INPUT.lines().enumerate() {
        w = line.len() as isize;
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                c => antenna_locations.entry(c).or_insert(vec![]).push((x, y)),
            }
        }
    }

    for (_, positions) in antenna_locations {
        for ((xu1, yu1), (xu2, yu2)) in positions.iter().tuple_combinations() {
            let x1 = *xu1 as isize;
            let y1 = *yu1 as isize;
            let x2 = *xu2 as isize;
            let y2 = *yu2 as isize;
            let diff_x = x2 - x1;
            let diff_y = y2 - y1;
            let d = gcd(diff_x.abs(), diff_y.abs());
            let diff_x = diff_x / d;
            let diff_y = diff_y / d;
            let mut n = 0;
            while x1 + diff_x * n < w
                && x1 + n * diff_x >= 0
                && y1 + n * diff_y < h
                && y1 + diff_y * n >= 0
            {
                let x = x1 + diff_x * n;
                let y = y1 + diff_y * n;
                anti_nodes.insert((x, y));
                n += 1;
            }
            let mut n = -1;
            while x1 + diff_x * n < w
                && x1 + n * diff_x >= 0
                && y1 + n * diff_y < h
                && y1 + diff_y * n >= 0
            {
                let x = x1 + diff_x * n;
                let y = y1 + diff_y * n;
                anti_nodes.insert((x, y));
                n -= 1;
            }
            println!("pair done")
        }
        println!("freq done")
    }

    println!("{}", anti_nodes.len());
    println!("{:?}", anti_nodes);
}

fn gcd(mut n1: isize, mut n2: isize) -> isize {
    let mut c = 0;
    if n1 < 1 || n2 < 1 {
        return 1;
    }
    while n1 % 2 == 0 && n2 % 2 == 0 {
        c += 1;
        n1 /= 2;
        n2 /= 2;
    }

    loop {
        if n1 == n2 {
            break;
        }

        if n1 > n2 {
            n1 = n1 - n2;
            while n1 % 2 == 0 {
                n1 /= 2;
            }
        }
        if n1 < n2 {
            n2 = n2 - n1;
            while n2 % 2 == 0 {
                n2 /= 2;
            }
        }
    }

    2_isize.pow(c) * n1
}
