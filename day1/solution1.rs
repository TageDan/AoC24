const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in INPUT.lines() {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        left.push(parts[0]);
        right.push(parts[1]);
    }
    left.sort_unstable();
    right.sort_unstable();
    for (x, y) in right.iter().zip(left.iter()) {
        result += (x - y).abs();
    }

    println!("{result}")
}
