mod game;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let parts = INPUT.split("\n\n").collect::<Vec<_>>();

    let mut map = game::Map2::from(parts[0]);
    let moves = parts[1]
        .lines()
        .flat_map(|x| x.chars())
        .map(|x| game::Move::from(x))
        .collect::<Vec<_>>();

    println!("\x1b[2J");

    'a: for m in moves {
        // println!(
        //     "\x1b[H\x1b[2KSum: {}\n\nMap:\n{}",
        //     map.boxes.iter().map(|x| x.gps_coord()).sum::<u32>(),
        //     map
        // );
        let n_player = map.player.moved(&m);
        let mut n_boxes = map.boxes.clone();
        if let Some((bi, _)) = map
            .boxes
            .iter()
            .enumerate()
            .find(|(_, x)| x.pos == n_player.pos || [x.pos[0] + 1, x.pos[1]] == n_player.pos)
        {
            let mut bis = vec![bi];
            loop {
                let mut n_bis = vec![];
                for bi in bis {
                    n_boxes[bi] = n_boxes[bi].pushed(&m);
                    map.boxes
                        .iter()
                        .enumerate()
                        .filter(|(i, x)| {
                            if *i == bi {
                                return false;
                            }
                            x.pos == n_boxes[bi].pos
                                || [x.pos[0] + 1, x.pos[1]] == n_boxes[bi].pos
                                || [n_boxes[bi].pos[0] + 1, n_boxes[bi].pos[1]] == x.pos
                        })
                        .for_each(|(x, _)| {
                            if !n_bis.contains(&x) {
                                n_bis.push(x)
                            }
                        });

                    if let Some(_) = map
                        .walls
                        .iter()
                        .filter(|x| {
                            x.pos[1] == n_boxes[bi].pos[1]
                                && (x.pos[0] == n_boxes[bi].pos[0]
                                    || x.pos[0] == n_boxes[bi].pos[0] + 1)
                        })
                        .next()
                    {
                        continue 'a;
                    }
                }

                if n_bis.is_empty() {
                    break;
                }

                bis = n_bis;
            }

            map.boxes = n_boxes;
            map.player = n_player;
        } else if let Some(_) = map
            .walls
            .iter()
            .filter(|x| [x.pos[0] / 2, x.pos[1]] == [n_player.pos[0] / 2, n_player.pos[1]])
            .next()
        {
            continue;
        } else {
            map.player = n_player
        }
        // std::thread::sleep_ms(10);
    }
    println!(
        "\x1b[HSum: {}\n\nMap:\n{}",
        map.boxes.iter().map(|x| x.gps_coord()).sum::<u32>(),
        map
    );
}
