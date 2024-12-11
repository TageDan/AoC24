const INPUT: &str = include_str!("input.txt");
use std::collections::HashMap;

use rayon::{
    self,
    iter::{IntoParallelRefIterator, ParallelIterator},
};

fn main() {
    let mut line = INPUT
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut mem = HashMap::new();
    let mut result = 0;
    for num in line {
        result += blink_n(75, num, &mut mem);
    }
    println!("{}", result);
}

fn blink_n(d: usize, num: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if d == 0 {
        return 1;
    }
    if let Some(x) = mem.get(&(d, num)) {
        return *x;
    }
    match num.to_string().as_str() {
        "0" => {
            let result = blink_n(d - 1, 1, mem);
            mem.insert((d, num), result);
            return result;
        }
        c if c.len() % 2 == 0 => {
            let (r1, r2) = c.split_at(c.len() / 2);
            let r1 = r1.parse().unwrap();
            let res1 = blink_n(d - 1, r1, mem);
            let r2 = r2.parse().unwrap();
            let res2 = blink_n(d - 1, r2, mem);
            mem.insert((d, num), res2 + res1);
            return res2 + res1;
        }
        _ => {
            let result = blink_n(d - 1, num * 2024, mem);
            mem.insert((d, num), result);
            return result;
        }
    }
}
