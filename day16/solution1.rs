use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("input.txt");

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Eq, PartialEq, Debug)]
struct State {
    pos: (i32, i32),
    dir: usize,
    turns: usize,
    steps: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost().cmp(&self.cost()))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}

impl State {
    fn new(pos: (i32, i32), dir: usize, turns: usize, steps: usize) -> Self {
        Self {
            pos,
            dir,
            turns,
            steps,
        }
    }

    fn cost(&self) -> usize {
        self.turns * 1000 + self.steps
    }
}

fn main() {
    let mut walls = vec![];
    let mut pos: (i32, i32) = (0, 0);
    let mut end = (0, 0);
    let dir: usize = 0;
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => walls.push((x as i32, y as i32)),
                '.' => (),
                'S' => pos = (x as i32, y as i32),
                'E' => end = (x as i32, y as i32),
                _ => panic!("fel"),
            }
        }
    }

    let best_path = lowest_score(pos, &walls, end, dir);

    println!("{best_path}")
}

fn lowest_score(pos: (i32, i32), walls: &Vec<(i32, i32)>, end: (i32, i32), dir: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut state = State::new(pos, dir, 0, 0);
    let mut visited_lowest_cost = HashMap::new();
    visited_lowest_cost.insert((state.pos, state.dir), 0_usize);
    heap.push(state);
    while !heap.is_empty() {
        state = heap.pop().unwrap();
        if state.pos == end {
            println!("{state:?}");
            return state.cost();
        }
        let dir = DIRS[rotate_left(state.dir)];
        let r_left = State {
            pos: (state.pos.0 + dir.0, state.pos.1 + dir.1),
            dir: rotate_left(state.dir),
            turns: state.turns + 1,
            steps: state.steps + 1,
        };
        let dir = DIRS[rotate_right(state.dir)];
        let r_right = State {
            pos: (state.pos.0 + dir.0, state.pos.1 + dir.1),
            dir: rotate_right(state.dir),
            turns: state.turns + 1,
            steps: state.steps + 1,
        };
        let dir = DIRS[state.dir];
        let forward = State {
            pos: (state.pos.0 + dir.0, state.pos.1 + dir.1),
            dir: state.dir,
            steps: state.steps + 1,
            turns: state.turns,
        };
        if (!visited_lowest_cost.contains_key(&(r_left.pos, r_left.dir))
            || r_left.cost() <= *visited_lowest_cost.get(&(r_left.pos, r_left.dir)).unwrap())
            && !walls.contains(&r_left.pos)
        {
            visited_lowest_cost.insert((r_left.pos, r_left.dir), r_left.cost());
            heap.push(r_left);
        }
        if (!visited_lowest_cost.contains_key(&(r_right.pos, r_right.dir))
            || r_right.cost()
                <= *visited_lowest_cost
                    .get(&(r_right.pos, r_right.dir))
                    .unwrap())
            && !walls.contains(&r_right.pos)
        {
            visited_lowest_cost.insert((r_right.pos, r_right.dir), r_right.cost());
            heap.push(r_right);
        }
        if (!visited_lowest_cost.contains_key(&(forward.pos, forward.dir))
            || forward.cost()
                <= *visited_lowest_cost
                    .get(&(forward.pos, forward.dir))
                    .unwrap())
            && !walls.contains(&forward.pos)
        {
            visited_lowest_cost.insert((forward.pos, forward.dir), forward.cost());
            heap.push(forward);
        }
    }
    panic!("no solution")
}

fn rotate_right(dir: usize) -> usize {
    if dir == 3 {
        0
    } else {
        dir + 1
    }
}

fn rotate_left(dir: usize) -> usize {
    if dir == 0 {
        3
    } else {
        dir - 1
    }
}
