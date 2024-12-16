use std::fmt::Display;

#[derive(Clone)]
pub struct Box {
    pub pos: [u32; 2],
}

impl Box {
    pub fn pushed(&self, dir: &Move) -> Self {
        match dir {
            Move::Right => Self {
                pos: [self.pos[0] + 1, self.pos[1]],
            },
            Move::Left => Self {
                pos: [self.pos[0] - 1, self.pos[1]],
            },
            Move::Down => Self {
                pos: [self.pos[0], self.pos[1] + 1],
            },
            Move::Up => Self {
                pos: [self.pos[0], self.pos[1] - 1],
            },
        }
    }

    pub fn gps_coord(&self) -> u32 {
        self.pos[0] + self.pos[1] * 100
    }
}

pub struct Wall {
    pub pos: [u32; 2],
}

pub struct Player {
    pub pos: [u32; 2],
}

impl Player {
    pub fn moved(&self, dir: &Move) -> Self {
        match dir {
            Move::Right => Self {
                pos: [self.pos[0] + 1, self.pos[1]],
            },
            Move::Left => Self {
                pos: [self.pos[0] - 1, self.pos[1]],
            },
            Move::Down => Self {
                pos: [self.pos[0], self.pos[1] + 1],
            },
            Move::Up => Self {
                pos: [self.pos[0], self.pos[1] - 1],
            },
        }
    }
}

pub struct Map {
    pub boxes: Vec<Box>,
    pub walls: Vec<Wall>,
    pub player: Player,
    pub w: u32,
    pub h: u32,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.boxes.iter().filter(|b| b.pos == [x, y]).count() != 0 {
                    map_str.push_str("\x1b[38;2;255;255;255mO");
                } else if self.walls.iter().filter(|b| b.pos == [x, y]).count() != 0 {
                    map_str.push_str("\x1b[38;2;150;150;150m#");
                } else if self.player.pos == [x, y] {
                    map_str.push_str("\x1b[38;2;255;100;0m@");
                } else {
                    map_str.push_str("\x1b[38;2;80;80;80m.");
                }
            }
            map_str.push('\n');
        }
        write!(f, "{}", map_str)
    }
}

impl From<&str> for Map2 {
    fn from(value: &str) -> Self {
        let mut walls = vec![];
        let mut boxes = vec![];
        let mut player = Player { pos: [0, 0] };
        let mut h = 0;
        let mut w = 0;
        for (y, line) in value.lines().enumerate() {
            h += 1;
            for (x, c) in line.chars().enumerate() {
                if h == 1 {
                    w += 1;
                }
                match c {
                    '#' => {
                        walls.push(Wall {
                            pos: [x as u32 * 2, y as u32],
                        });
                        walls.push(Wall {
                            pos: [x as u32 * 2 + 1, y as u32],
                        })
                    }
                    'O' => boxes.push(Box {
                        pos: [x as u32 * 2, y as u32],
                    }),
                    '@' => {
                        player = Player {
                            pos: [x as u32 * 2, y as u32],
                        }
                    }
                    '.' => (),
                    _ => panic!("unexpected value: {}", c),
                }
            }
        }

        Self {
            boxes,
            walls,
            player,
            h,
            w: w * 2,
        }
    }
}

pub struct Map2 {
    pub boxes: Vec<Box>,
    pub walls: Vec<Wall>,
    pub player: Player,
    pub w: u32,
    pub h: u32,
}

impl Display for Map2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_str = String::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.boxes.iter().filter(|b| b.pos == [x, y]).count() != 0 {
                    map_str.push_str("\x1b[38;2;255;255;255m[");
                } else if self
                    .boxes
                    .iter()
                    .filter(|b| b.pos[0] + 1 == x && b.pos[1] == y)
                    .count()
                    != 0
                {
                    map_str.push_str("\x1b[38;2;255;255;255m]");
                } else if self.walls.iter().filter(|b| b.pos == [x, y]).count() != 0 {
                    map_str.push_str("\x1b[38;2;150;150;150m#");
                } else if self.player.pos == [x, y] {
                    map_str.push_str("\x1b[38;2;255;100;0m@");
                } else {
                    map_str.push_str("\x1b[38;2;80;80;80m.");
                }
            }
            map_str.push('\n');
        }
        write!(f, "{}", map_str)
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut walls = vec![];
        let mut boxes = vec![];
        let mut player = Player { pos: [0, 0] };
        let mut h = 0;
        let mut w = 0;
        for (y, line) in value.lines().enumerate() {
            h += 1;
            for (x, c) in line.chars().enumerate() {
                if h == 1 {
                    w += 1;
                }
                match c {
                    '#' => walls.push(Wall {
                        pos: [x as u32, y as u32],
                    }),
                    'O' => boxes.push(Box {
                        pos: [x as u32, y as u32],
                    }),
                    '@' => {
                        player = Player {
                            pos: [x as u32, y as u32],
                        }
                    }
                    '.' => (),
                    _ => panic!("unexpected value: {}", c),
                }
            }
        }

        Self {
            boxes,
            walls,
            player,
            h,
            w,
        }
    }
}

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            _ => panic!("unexpected value: {}", value),
        }
    }
}
