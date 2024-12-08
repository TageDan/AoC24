use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;
    let mut width = 0;
    let mut height = 0;
    let mut grid = vec![];

    for line in INPUT.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
        width = width.max(line.len());
        height += 1;
        for w in line.chars().tuple_windows() {
            match w {
                ('X', 'M', 'A', 'S') => result += 1,
                ('S', 'A', 'M', 'X') => result += 1,
                (_, _, _, _) => (),
            }
        }
    }

    for x in 0..width {
        INPUT
            .lines()
            .map(|l| l.get(x..x + 1).unwrap())
            .tuple_windows()
            .for_each(|w| match w {
                ("X", "M", "A", "S") => result += 1,
                ("S", "A", "M", "X") => result += 1,
                _ => (),
            });
    }

    for off in -(width as isize)..((height + width) as isize) {
        let h = off;
        let w = width as isize;
        let mut string = String::new();
        let mut string2 = String::new();
        for i in 0..w {
            if h + i >= 0 && h + i < height as isize {
                string.push(grid[(h + i) as usize][i as usize]);
            }
            if h - i >= 0 && h - i < height as isize {
                string2.push(grid[(h - i) as usize][i as usize]);
            }
        }

        string.chars().tuple_windows().for_each(|w| match w {
            ('X', 'M', 'A', 'S') => {
                result += 1;
            }
            ('S', 'A', 'M', 'X') => {
                result += 1;
            }
            _ => (),
        });
        string2.chars().tuple_windows().for_each(|w| match w {
            ('X', 'M', 'A', 'S') => {
                result += 1;
            }
            ('S', 'A', 'M', 'X') => {
                result += 1;
            }
            _ => (),
        });
    }

    println!("{result}")
}
