const INPUT: &str = include_str!("input.txt");

struct Machine {
    b_button: [u32; 2],
    a_button: [u32; 2],
    prize: [u32; 2],
}

impl Machine {
    fn cost_to_win(&self) -> Option<u32> {
        let solutions_x = diophantine_solutions(self.a_button[0], self.b_button[0], self.prize[0]);
        let solutions_y = diophantine_solutions(self.a_button[1], self.b_button[1], self.prize[1]);
        solutions_x
            .iter()
            .filter_map(|x| {
                if solutions_y.contains(x) {
                    Some(x[0] * 3 + x[1])
                } else {
                    None
                }
            })
            .min()
    }
}

fn diophantine_solutions(a: u32, b: u32, c: u32) -> Vec<[u32; 2]> {
    let mut solutions = vec![];
    for x in 0..=100 {
        for y in 0..=100 {
            if a * x + b * y == c {
                solutions.push([x, y]);
            }
        }
    }

    solutions
}

fn gcd(mut n1: u32, mut n2: u32) -> u32 {
    let mut c = 0;
    if n1 < 1 || n2 < 1 {
        return 1;
    }
    while n1 % 2 == 0 && n2 % 2 == 0 {
        c += 1;
        n1 /= 2;
        n2 /= 2;
    }

    loop {
        if n1 == n2 {
            break;
        }

        if n1 > n2 {
            n1 = n1 - n2;
            while n1 % 2 == 0 {
                n1 /= 2;
            }
        }
        if n1 < n2 {
            n2 = n2 - n1;
            while n2 % 2 == 0 {
                n2 /= 2;
            }
        }
    }

    2_u32.pow(c) * n1
}

fn main() {
    let mut machines = vec![];
    let mut a = [0, 0];
    let mut b = [0, 0];
    let mut prize = [0, 0];
    let mut i = 0;
    for line in INPUT.lines() {
        match i % 4 {
            0 => a = parse(line),
            1 => b = parse(line),
            2 => prize = parse(line),
            _ => machines.push(Machine {
                a_button: a.clone(),
                b_button: b.clone(),
                prize: prize.clone(),
            }),
        }

        i = (i + 1) % 4;
    }

    let result = machines
        .iter()
        .filter_map(|machine| machine.cost_to_win())
        .sum::<u32>();

    println!("{result}");
}

fn parse(line: &str) -> [u32; 2] {
    let mut parts = line.split(": ");
    parts.next();
    let nums = parts.next().unwrap();
    let (x, y) = nums.split_once(", ").unwrap();
    return [
        x[2..].parse::<u32>().unwrap(),
        y[2..].parse::<u32>().unwrap(),
    ];
}
