const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut dotted_string: Vec<String> = INPUT
        .chars()
        .enumerate()
        .flat_map(|(i, x)| {
            if let Some(x) = x.to_string().parse::<usize>().ok() {
                let this_file: Vec<_> = match i % 2 {
                    0 => {
                        let index = i / 2;
                        (0..x).map(|x| format!("{}", index)).collect()
                    }
                    _ => (0..x).map(|x| String::from(".")).collect(),
                };
                return this_file.into_iter();
            } else {
                vec![].into_iter()
            }
        })
        .collect();

    let mut reversed_undotted_string: Vec<usize> = dotted_string
        .clone()
        .into_iter()
        .rev()
        .filter(|x| *x != ".")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut current_file = vec![];
    let mut current_id = 0;

    let mut remove_from = 0;

    let loading = LoadingBar::new(reversed_undotted_string.len());

    for (idx, id) in reversed_undotted_string.into_iter().enumerate() {
        if id == current_id || current_file.is_empty() {
            current_file.push(id);
            current_id = id;
        } else {
            let mut check = true;
            let mut count = 0;
            let mut start = None;
            for (i, c) in dotted_string.iter().enumerate() {
                if c.as_str() == &format!("{}", current_id) {
                    if check {
                        start = None;
                        count = 0;
                    }

                    remove_from = i;

                    break;
                }
                if check && c.as_str() == "." {
                    if start.is_none() {
                        start = Some(i);
                        count = 1;
                        if count >= current_file.len() {
                            check = false;
                        };
                    } else {
                        count += 1;
                        if count >= current_file.len() {
                            check = false;
                        };
                    }
                } else if check {
                    start = None;
                    count = 0;
                }
            }

            if let Some(x) = start {
                for pos in x..(x + count) {
                    dotted_string[pos] = current_id.to_string();
                }
                for pos in remove_from..(remove_from + count) {
                    dotted_string[pos] = String::from(".");
                }
            }
            current_file = vec![id];
            current_id = id;
        }
    }

    println!("{}", dotted_string.join(""));
    let result = dotted_string
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if x == "." {
                return None;
            } else {
                return Some(x.parse::<usize>().unwrap() * i);
            }
        })
        .sum::<usize>();
    println!("{result}");
}
