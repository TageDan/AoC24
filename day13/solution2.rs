const INPUT: &str = include_str!("input.txt");

struct Machine {
    b_button: [i64; 2],
    a_button: [i64; 2],
    prize: [i64; 2],
}

impl Machine {
    fn cost_to_win(&self) -> Option<i64> {
        let a1 = self.a_button[0];
        let a2 = self.a_button[1];
        let b1 = self.b_button[0];
        let b2 = self.b_button[1];
        let p1 = self.prize[0];
        let p2 = self.prize[1];

        let area_ab = (a2 * b1 - a1 * b2).abs();
        let area_ap = (a2 * p1 - p2 * a1).abs();
        if area_ab == 0 {
            return None;
        }
        let lb = area_ap / area_ab;
        let la = (p1 - b1 * lb).abs() / a1;

        if la * a1 + lb * b1 != p1 || la * a2 + lb * b2 != p2 {
            return None;
        }

        Some(la * 3 + lb)
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
                prize: [prize[0] + 10000000000000, prize[1] + 10000000000000],
                // prize: prize.clone(),
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
