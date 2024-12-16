const INPUT: &str = include_str!("input.txt");

struct Machine {
    b_button: [i64; 2],
    a_button: [i64; 2],
    prize: [i64; 2],
}

impl Machine {
    fn cost_to_win(&self) -> Option<i64> {
        let solutions_x = solve_simultaneous(
            self.a_button[0],
            self.b_button[0],
            self.prize[0],
            self.a_button[1],
            self.b_button[1],
            self.prize[1],
        );
        println!("done with gathering possible solutions");
        solutions_x.iter().map(|x| x[0] * 3 + x[1]).min()
    }
}

fn solve_single_diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    let (gcd, x0, y0) = extended_gcd(a, b);
    if c % gcd != 0 {
        None // No solutions
    } else {
        let scale = c / gcd;
        Some((x0 * scale, y0 * scale, b / gcd, a / gcd))
    }
}

fn solve_simultaneous(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> Vec<[i64; 2]> {
    if let Some((x1, y1, step_x1, step_y1)) = solve_single_diophantine(a1, b1, c1) {
        let new_a = a2 * step_x1 + b2 * step_y1;
        let new_c = c2 - a2 * x1 - b2 * y1;

        if new_a == 0 {
            if new_c == 0 {
                return vec![];
            } else {
                return vec![];
            }
        }

        if let Some((k1, _, step_k1, _)) = solve_single_diophantine(new_a, 0, new_c) {
            let mut solutions = Vec::new();

            let mut k = 0;
            while 

            for k in -10..=10 {
                let final_k1 = k1 + k * step_k1;
                let x = x1 + final_k1 * step_x1;
                let y = y1 + final_k1 * step_y1;
                solutions.push([x, y]);
            }
            return solutions;
        }
    }

    vec![]
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
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
                // prize: [prize[0] + 10000000000000, prize[1] + 10000000000000],
                prize: prize.clone(),
            }),
        }

        i = (i + 1) % 4;
    }

    let result = machines
        .iter()
        .filter_map(|machine| machine.cost_to_win())
        .sum::<i64>();

    println!("{result}");
}

fn parse(line: &str) -> [i64; 2] {
    let mut parts = line.split(": ");
    parts.next();
    let nums = parts.next().unwrap();
    let (x, y) = nums.split_once(", ").unwrap();
    return [
        x[2..].parse::<i64>().unwrap(),
        y[2..].parse::<i64>().unwrap(),
    ];
}
