const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut line = INPUT
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("{:?}", line);
    for i in 0..25 {
        line = blink(line);
        if i < 6 {
            println!("{:?}", line);
        }
    }
    println!("{}", line.len())
}

fn blink(line: Vec<usize>) -> Vec<usize> {
    let mut new_line = Vec::with_capacity(line.len() * 2);
    for x in line {
        match x.to_string().as_str() {
            "0" => new_line.push(1),
            c if c.len() % 2 == 0 => {
                let (r1, r2) = c.split_at(c.len() / 2);
                new_line.push(r1.parse::<usize>().unwrap());
                new_line.push(r2.parse::<usize>().unwrap());
            }
            _ => new_line.push(x * 2024),
        }
    }

    new_line
}
