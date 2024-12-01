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
    for x in left.iter() {
        result += x * right
            .iter()
            .fold(0, |acc, y| if y == x { acc + 1 } else { acc })
    }

    println!("{result}")
}
