const INPUT: &str = include_str!("input.txt");

fn main() {
    let dotted_string: Vec<String> = INPUT
        .chars()
        .enumerate()
        .flat_map(|(i, x)| {
            print!("{}", x);
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

    let mut undotted_string: Vec<usize> = dotted_string
        .clone()
        .into_iter()
        .filter(|x| *x != ".")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut result_string = vec![];
    for c in dotted_string.iter().take(undotted_string.len()) {
        match c.as_str() {
            "." => result_string.push(undotted_string.pop().unwrap_or(0)),
            c => result_string.push(c.parse::<usize>().unwrap()),
        }
    }

    let result = result_string
        .iter()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum::<usize>();

    print!("{result}");
}
