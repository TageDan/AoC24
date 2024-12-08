const INPUT: &str = include_str!("input.txt");

fn main() {
    let result = INPUT
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(":");
            let result = parts.next().unwrap().parse::<usize>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let acc = numbers[0];
            if check(&numbers[1..], result, acc) {
                Some(result)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{result}")
}

fn check(numbers: &[usize], result: usize, acc: usize) -> bool {
    if numbers.len() == 1 {
        return acc + numbers[0] == result || acc * numbers[0] == result;
    };
    return check(&numbers[1..], result, acc * numbers[0])
        || check(&numbers[1..], result, acc + numbers[0]);
}
