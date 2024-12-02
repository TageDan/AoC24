use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;

    for line in INPUT.lines() {
        let number = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        if check(&number, None) {
            result += 1;
        }
    }
    println!("{result}")
}

#[inline]
fn check(list: &[isize], dont_check: Option<usize>) -> bool {
    let mut l: isize = -1;
    let mut d: isize = 0;

    for (i, val) in list
        .iter()
        .enumerate()
        .filter(|(x, _)| dont_check.is_none() || dont_check.unwrap() != *x)
    {
        if l != -1 {
            if d != 0 {
                if (val - l).abs() <= 3 && (val - l).abs() >= 1 && (val - l).signum() == d.signum()
                {
                    d = val - l;
                    l = *val;
                } else {
                    if dont_check.is_none() {
                        return check(list, Some(i))
                            || check(list, Some(i - 1))
                            || check(list, Some(0));
                    } else {
                        return false;
                    }
                }
            } else {
                if (val - l).abs() <= 3 && (val - l).abs() >= 1 {
                    d = val - l;
                    l = *val;
                } else {
                    if dont_check.is_none() {
                        return check(list, Some(i)) || check(list, Some(i - 1));
                    } else {
                        return false;
                    }
                }
            }
        } else {
            l = *val;
        }
    }
    return true;
}

// other solution

// fn main() {
//     let mut result = 0;

//     for line in INPUT.lines() {
//         let number = line.split_whitespace().map(|x| x.parse::<isize>().unwrap());
//         let len = number.clone().count();
//         if check(&number.clone().collect::<Vec<_>>()) {
//             result += 1;
//         } else {
//             for iterator in number.combinations(len - 1) {
//                 if check(&iterator) {
//                     result += 1;
//                     break;
//                 }
//             }
//         }
//     }
//     println!("{result}")
// }

// #[inline]
// fn check(list: &[isize]) -> bool {
//     let mut l: isize = -1;
//     let mut d: isize = 0;

//     for (i, val) in list.iter().enumerate() {
//         if l != -1 {
//             if d != 0 {
//                 if (val - l).abs() <= 3 && (val - l).abs() >= 1 && (val - l).signum() == d.signum()
//                 {
//                     d = val - l;
//                     l = *val;
//                 } else {
//                     return false;
//                 }
//             } else {
//                 if (val - l).abs() <= 3 && (val - l).abs() >= 1 {
//                     d = val - l;
//                     l = *val;
//                 } else {
//                     return false;
//                 }
//             }
//         } else {
//             l = *val;
//         }
//     }
//     return true;
// }
