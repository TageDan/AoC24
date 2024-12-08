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
            let xn1 = x1 - diff_x;
            let yn1 = y1 - diff_y;
            let xn2 = x2 + diff_x;
            let yn2 = y2 + diff_y;
            if xn1 < w && xn1 >= 0 && yn1 < h && yn1 >= 0 {
                anti_nodes.insert((xn1, yn1));
            }
            if xn2 < w && xn2 >= 0 && yn2 < h && yn2 >= 0 {
                anti_nodes.insert((xn2, yn2));
            }
        }
    }

    println!("{}", anti_nodes.len());
    println!("{:?}", anti_nodes);
}
