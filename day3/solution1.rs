use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let reg = Regex::new(r"\([0-9]{3}\,[0-9]{3}\)").unwrap();

    let mut results = vec![];
    for (_, [string]) in reg.captures_iter(INPUT).map(|c| c.extract()) {
        results.push(string);
    }
    dbg!(results);
}
