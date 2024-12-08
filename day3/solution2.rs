const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut do_bool = true;
    let mut result = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();
        let mut match_current = String::new();
        while chars.peek().is_some() {
            match chars.next() {
                Some('m') => match_current = "m".into(),
                Some('d') => match_current = "d".into(),
                Some(c) => match_current.push(c),
                None => (),
            }
            match match_current {
                ref mut s
                    if s.starts_with("mul(")
                        && s.ends_with(")")
                        && s.chars().filter(|x| *x == ',').count() == 1 =>
                {
                    let mut parts = s.split(',');
                    let mut part1 = parts.next().unwrap();
                    let mut part2 = parts.next().unwrap();
                    if do_bool {
                        if let Ok(p1) = part1[4..].parse::<usize>() {
                            if let Ok(p2) = part2[..part2.len() - 1].parse::<usize>() {
                                result += p1 * p2;
                            }
                        }
                    }
                }
                ref mut s if s == "do()" => {
                    do_bool = true;
                    *s = String::new();
                }
                ref mut s if s == "don't()" => {
                    do_bool = false;
                    *s = String::new();
                }
                _ => (),
            }
        }
    }
    println!("{result}")
}
