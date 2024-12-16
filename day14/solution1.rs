use regex::{self, Regex};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: [i32; 2],
    vel: [i32; 2],
}

impl Robot {
    fn step(&mut self, seconds: i32, room: &Room) {
        self.pos[0] = (self.pos[0] + self.vel[0] * seconds).rem_euclid(room.w);
        self.pos[1] = (self.pos[1] + self.vel[1] * seconds).rem_euclid(room.h);
    }

    fn parse(line: &str) -> Self {
        let reg = Regex::new(r"p=(-*[0-9]*),(-*[0-9]*)").unwrap();
        let reg2 = Regex::new(r"v=(-*[0-9]*),(-*[0-9]*)").unwrap();
        let pos = reg.captures(line).unwrap();
        let vel = reg2.captures(line).unwrap();

        return Self {
            pos: [
                pos[1].parse::<i32>().unwrap(),
                pos[2].parse::<i32>().unwrap(),
            ],
            vel: [
                vel[1].parse::<i32>().unwrap(),
                vel[2].parse::<i32>().unwrap(),
            ],
        };
    }
}

struct Room {
    w: i32,
    h: i32,
}

fn main() {
    let room = Room { w: 101, h: 103 };
    let mut quads = [0, 0, 0, 0];
    let mut robots = vec![];
    for line in INPUT.lines() {
        let mut robot = Robot::parse(line);
        robot.step(100, &room);
        robots.push(robot.clone());
        if robot.pos[0] != (room.w) / 2 && robot.pos[1] != (room.h) / 2 {
            let quad = ((robot.pos[0] > (room.w) / 2) as usize)
                | ((robot.pos[1] > (room.h) / 2) as usize) << 1;
            quads[quad] += 1;
        }
    }

    for y in 0..room.h {
        for x in 0..room.w {
            if x == (room.w) / 2 || y == (room.h) / 2 {
                print!(" ");
                continue;
            };
            let mut robotc = 0;
            for r in &robots {
                if r.pos[0] == x && r.pos[1] == y {
                    robotc += 1;
                }
            }
            match robotc {
                0 => print!("."),
                c => print!("{}", c),
            }
        }
        println!("");
    }

    println!("{}", quads.iter().product::<i32>());
}
