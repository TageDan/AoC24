use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

struct Rule {
    first: usize,
    second: usize,
}

impl Rule {
    fn check(&self, list: &Vec<usize>) -> bool {
        for (i, l1) in list.iter().enumerate() {
            if *l1 == self.second {
                for l2 in &list[i..] {
                    if *l2 == self.first {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn new(first: usize, second: usize) -> Self {
        return Self { first, second };
    }
}

fn main() {
    let mut result = 0;
    let mut rules: Vec<Rule> = vec![];
    let mut adding_rules = true;

    for line in INPUT.lines() {
        if line.trim().is_empty() {
            adding_rules = false;
            continue;
        }
        if adding_rules {
            let mut parts = line.split('|');
            rules.push(Rule::new(
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            ));
        } else {
            let mut update: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().expect(&format!("{x}")))
                .collect();

            let mut valid = true;
            for rule in &rules {
                if !rule.check(&update) {
                    valid = false;
                };
            }

            if !valid {
                update.sort_unstable_by(|a, b| {
                    for rule in rules.iter() {
                        if rule.second == *a {
                            if rule.first == *b {
                                return std::cmp::Ordering::Greater;
                            }
                        }
                    }
                    return std::cmp::Ordering::Less;
                });

                result += update.get(update.len() / 2).unwrap();

                println!("{update:?}");
            }
        }
    }

    println!("{result}")
}