const INPUT: &str = include_str!("input.txt");
use numext_fixed_uint::U2048;
use std::collections::HashMap;

fn main() {
    let mut line = INPUT
        .trim()
        .split(" ")
        .filter_map(|x| x.parse::<u128>().ok())
        .collect::<Vec<_>>();

    let mut mem = HashMap::new();
    let mut result = U2048::zero();
    for num in line {
        result = result + blink_n(U2048::from(400_u32), U2048::from(num), &mut mem);
    }
    println!("{}", result);
}

fn blink_n(d: U2048, num: U2048, mem: &mut HashMap<(U2048, U2048), U2048>) -> U2048 {
    if d == U2048::zero() {
        return U2048::one();
    }
    if let Some(x) = mem.get(&(d.clone(), num.clone())) {
        return x.clone();
    }
    match num.clone().to_string().as_str() {
        "0" => {
            let result = blink_n(d.clone() - U2048::one(), U2048::one(), mem);
            mem.insert((d.clone(), num), result.clone());
            return result;
        }
        c if c.len() % 2 == 0 => {
            let (r1, r2) = c.split_at(c.len() / 2);
            let r1: u128 = r1.parse().unwrap();
            let res1 = blink_n(d.clone() - U2048::one(), U2048::from(r1), mem);
            let r2: u128 = r2.parse().unwrap();
            let res2 = blink_n(d.clone() - U2048::one(), U2048::from(r2), mem);
            mem.insert((d.clone(), num), res2.clone() + res1.clone());
            return res2 + res1;
        }
        _ => {
            let result = blink_n(
                d.clone() - U2048::one(),
                num.clone() * U2048::from(2024_u32),
                mem,
            );
            mem.insert((d.clone(), num.clone()), result.clone());
            return result.clone();
        }
    }
}
