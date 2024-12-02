const INPUT: &str = include_str!("input.txt");
use itertools::Itertools;

//hej

fn main() {
    let mut result = 0;
    let mut count = false;

    for line in INPUT.lines() {
        count = true;
        let mut last: Option<isize> = None;
        let mut diff: Option<isize> = None;
        let mut count = true;
        for number in line.split_whitespace().map(|x| x.parse::<isize>().unwrap()) {
            if let Some(l) = last {
                if let Some(d) = diff {
                    if (number - l).signum() == d.signum()
                        && (number - l).abs() <= 3
                        && (number - l).abs() >= 1
                    {
                        diff = Some(number - l);
                        last = Some(number);
                    } else {
                        count = false;
                    }
                } else {
                    if (number - l).abs() <= 3 && (number - l).abs() >= 1 {
                        diff = Some(number - l);
                        last = Some(number);
                    } else {
                        count = false;
                    }
                }
            } else {
                last = Some(number);
            }
        }
        if count {
            result += 1;
        }
    }

    println!("{result}")
}
