mod game;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let parts = INPUT.split("\n\n").collect::<Vec<_>>();

    let mut map = game::Map::from(parts[0]);
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
        if let Some((mut bi, _)) = map
            .boxes
            .iter()
            .enumerate()
            .find(|(_, x)| x.pos == n_player.pos)
        {
            n_boxes[bi] = n_boxes[bi].pushed(&m);
            if let Some(_) = map.walls.iter().filter(|x| x.pos == n_boxes[bi].pos).next() {
                continue;
            }
            while let Some((bi2, _)) = map
                .boxes
                .iter()
                .enumerate()
                .find(|(_, x)| x.pos == n_boxes[bi].pos)
            {
                bi = bi2;
                n_boxes[bi] = n_boxes[bi].pushed(&m);
                if let Some(_) = map.walls.iter().filter(|x| x.pos == n_boxes[bi].pos).next() {
                    continue 'a;
                }
            }

            map.boxes = n_boxes;
            map.player = n_player;
        } else if let Some(_) = map.walls.iter().filter(|x| x.pos == n_player.pos).next() {
            continue;
        } else {
            map.player = n_player
        }
    }
    println!(
        "\x1b[HSum: {}\n\nMap:\n{}",
        map.boxes.iter().map(|x| x.gps_coord()).sum::<u32>(),
        map
    );
}
