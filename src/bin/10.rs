use std::{collections::HashSet, iter};

use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(10);

type Int = u32;

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Pipe {
    fn compose(a: Direction, b: Direction) -> Option<Self> {
        match (a, b) {
            (Direction::N, Direction::S) | (Direction::S, Direction::N) => Some(Self::NS),
            (Direction::N, Direction::E) | (Direction::E, Direction::N) => Some(Self::NE),
            (Direction::N, Direction::W) | (Direction::W, Direction::N) => Some(Self::NW),
            (Direction::S, Direction::E) | (Direction::E, Direction::S) => Some(Self::SE),
            (Direction::S, Direction::W) | (Direction::W, Direction::S) => Some(Self::SW),
            (Direction::E, Direction::W) | (Direction::W, Direction::E) => Some(Self::WE),
            _ => None,
        }
    }

    fn is_connected_to(&self, direction: Direction) -> bool {
        match self {
            Pipe::NS => matches!(direction, Direction::N | Direction::S),
            Pipe::WE => matches!(direction, Direction::W | Direction::E),
            Pipe::NE => matches!(direction, Direction::N | Direction::E),
            Pipe::NW => matches!(direction, Direction::N | Direction::W),
            Pipe::SW => matches!(direction, Direction::S | Direction::W),
            Pipe::SE => matches!(direction, Direction::S | Direction::E),
            Pipe::Ground => false,
            Pipe::Start => panic!("Pipe::Start connections are unknown"),
        }
    }

    fn pipe(&self, enter_by: Direction) -> Option<Direction> {
        match (self, enter_by) {
            (Pipe::NS, Direction::N) => Some(Direction::S),
            (Pipe::NS, Direction::S) => Some(Direction::N),

            (Pipe::WE, Direction::E) => Some(Direction::W),
            (Pipe::WE, Direction::W) => Some(Direction::E),

            (Pipe::NE, Direction::N) => Some(Direction::E),
            (Pipe::NE, Direction::E) => Some(Direction::N),

            (Pipe::NW, Direction::N) => Some(Direction::W),
            (Pipe::NW, Direction::W) => Some(Direction::N),

            (Pipe::SW, Direction::S) => Some(Direction::W),
            (Pipe::SW, Direction::W) => Some(Direction::S),

            (Pipe::SE, Direction::S) => Some(Direction::E),
            (Pipe::SE, Direction::E) => Some(Direction::S),

            (Pipe::Start, _) => panic!("Pipe::Start connections are unknown"),
            _ => None,
        }
    }
}

/// 0-based 2d position. S is positive Y, E is positive X.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn north(self) -> Option<Self> {
        let Self { x, y } = self;
        let y = y.checked_sub(1)?;
        Some(Self { x, y })
    }

    fn south(self) -> Self {
        let Self { x, y } = self;
        let y = y + 1;
        Self { x, y }
    }

    fn west(self) -> Option<Self> {
        let Self { x, y } = self;
        let x = x.checked_sub(1)?;
        Some(Self { x, y })
    }

    fn east(self) -> Self {
        let Self { x, y } = self;
        let x = x + 1;
        Self { x, y }
    }

    fn move_to(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::N => self.north(),
            Direction::S => Some(self.south()),
            Direction::E => Some(self.east()),
            Direction::W => self.west(),
        }
    }
}

struct Pipes(Vec<Vec<Pipe>>);

impl Pipes {
    fn get(&self, pos: Pos) -> Option<Pipe> {
        self.0.get(pos.y).and_then(|line| line.get(pos.x)).copied()
    }

    fn set(&mut self, pos: Pos, pipe: Pipe) {
        let target = self.0.get_mut(pos.y).and_then(|line| line.get_mut(pos.x));
        if let Some(target) = target {
            *target = pipe;
        }
    }

    fn size(&self) -> (usize, usize) {
        let w = self.0.first().unwrap().len();
        let h = self.0.len();
        (w, h)
    }
}

fn parse_input(input: &str) -> (Pos, Pipes) {
    let mut start_pos = None;

    let pipes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| match c {
                    b'.' => Pipe::Ground,
                    b'|' => Pipe::NS,
                    b'-' => Pipe::WE,
                    b'L' => Pipe::NE,
                    b'J' => Pipe::NW,
                    b'7' => Pipe::SW,
                    b'F' => Pipe::SE,
                    b'S' => {
                        start_pos = Some(Pos { x, y });
                        Pipe::Start
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    (start_pos.unwrap(), Pipes(pipes))
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let (start_pos, pipes) = parse_input(input);

    let mut pos = start_pos;

    let mut next_dir = {
        [Direction::N, Direction::S, Direction::E, Direction::W]
            .into_iter()
            .find(|&dir| {
                let Some(pos) = start_pos.move_to(dir) else {
                    return false;
                };

                let Some(pipe) = pipes.get(pos) else {
                    return false;
                };

                pipe.is_connected_to(dir.opposite())
            })
            .unwrap()
    };

    let mut step = 0;
    loop {
        step += 1;
        pos = pos.move_to(next_dir).unwrap();
        if pos == start_pos {
            break;
        }
        next_dir = pipes.get(pos).unwrap().pipe(next_dir.opposite()).unwrap();
    }

    Some(step / 2)
}

/* -------------------------------------------------------------------------- */

// pub fn part_two(input: &str) -> Option<Int> {
//     let (start_pos, mut pipes) = parse_input(input);

//     let mut pos = start_pos;

//     let first_dir = {
//         [Direction::N, Direction::S, Direction::E, Direction::W]
//             .into_iter()
//             .find(|&dir| {
//                 let Some(pos) = start_pos.move_to(dir) else {
//                     return false;
//                 };

//                 let Some(pipe) = pipes.get(pos) else {
//                     return false;
//                 };

//                 pipe.is_connected_to(dir.opposite())
//             })
//             .unwrap()
//     };

//     let mut loop_shell = HashSet::new();
//     let mut next_dir = first_dir;

//     let last_dir = loop {
//         loop_shell.insert(pos);
//         pos = pos.move_to(next_dir).unwrap();
//         if pos == start_pos {
//             break next_dir.opposite();
//         }
//         next_dir = pipes.get(pos).unwrap().pipe(next_dir.opposite()).unwrap();
//     };

//     pipes.set(start_pos, Pipe::compose(first_dir, last_dir).unwrap());

//     let (w, h) = pipes.size();

//     let mut inside_count = 0;
//     for y in 0..h {
//         for x in 0..w {
//             let pos = Pos { x, y };
//             // dbg!(pos);
//             if loop_shell.contains(&pos) {
//                 continue;
//             }

//             // A point is inside a polygon if a line between this point and a point far away
//             // hit the outline of the polygon a odd number of times.

//             let mut pos = pos;
//             let mut hit_outline = 0;
//             while let Some(north) = pos.north() {
//                 pos = north;

//                 if !loop_shell.contains(&pos) {
//                     continue;
//                 }

//                 let pipe = pipes.get(pos).unwrap();

//                 // Those shape count as 0
//                 //   ┐     ┌
//                 //   | and |
//                 //   ┘     └
//                 //
//                 // Those count as 1
//                 //   ┌     ┐
//                 //   | and |
//                 //   ┘     └

//                 match pipe {
//                     Pipe::NE => loop {
//                         pos = pos.north().unwrap();
//                         match pipes.get(pos).unwrap() {
//                             Pipe::SE => {
//                                 // ┌
//                                 // |
//                                 // └
//                                 break;
//                             }
//                             Pipe::SW => {
//                                 // ┐
//                                 // |
//                                 // └
//                                 hit_outline += 1;
//                                 break;
//                             }
//                             _ => continue,
//                         }
//                     },
//                     Pipe::NW => loop {
//                         pos = pos.north().unwrap();
//                         match pipes.get(pos).unwrap() {
//                             Pipe::SW => {
//                                 // ┐
//                                 // |
//                                 // ┘
//                                 break;
//                             }
//                             Pipe::SE => {
//                                 // ┌
//                                 // |
//                                 // ┘
//                                 hit_outline += 1;
//                                 break;
//                             }
//                             _ => continue,
//                         }
//                     },

//                     _ => {
//                         hit_outline += 1;
//                     }
//                 }
//             }

//             if hit_outline.is_odd() {
//                 inside_count += 1;
//             }
//         }
//     }

//     Some(inside_count)
// }

pub fn part_two(input: &str) -> Option<Int> {
    let (start_pos, mut pipes) = parse_input(input);

    let mut loop_outline = HashSet::new();

    {
        let first_dir = {
            [Direction::N, Direction::S, Direction::E, Direction::W]
                .into_iter()
                .find(|&dir| {
                    let Some(pos) = start_pos.move_to(dir) else {
                        return false;
                    };

                    let Some(pipe) = pipes.get(pos) else {
                        return false;
                    };

                    pipe.is_connected_to(dir.opposite())
                })
                .unwrap()
        };

        let mut next_dir = first_dir;
        let mut pos = start_pos;

        let last_dir = loop {
            loop_outline.insert(pos);
            pos = pos.move_to(next_dir).unwrap();
            if pos == start_pos {
                break next_dir.opposite();
            }
            next_dir = pipes.get(pos).unwrap().pipe(next_dir.opposite()).unwrap();
        };

        pipes.set(start_pos, Pipe::compose(first_dir, last_dir).unwrap());
    }

    let (w, h) = pipes.size();

    let mut inside = HashSet::new();
    for y in 0..h {
        'x: for x in 0..w {
            let current_pos = Pos { x, y };

            if loop_outline.contains(&current_pos) {
                // outline positions are not inside
                continue;
            }

            // A point is inside a polygon if a line between this point and a point far away
            // hit the outline of the polygon a odd number of times.

            let mut points = iter::successors(Some(current_pos), |p| p.north());

            points.next(); // skip the current_pos

            let mut hit_outline = 0;
            while let Some(p) = points.next() {
                if !loop_outline.contains(&p) {
                    // We encounter a point we already know if it's inside or outside.
                    let is_inside = inside.contains(&p);
                    if hit_outline.is_even() == is_inside {
                        // if even == true we are on the same side, so insert if is_inside is true.
                        // if event == false we are on opposite side, so insert if is_inside is false.
                        inside.insert(current_pos);
                    }

                    continue 'x;
                }

                let pipe = pipes.get(p).unwrap();

                // Those shape count as 0
                //   ┐     ┌
                //   | and |
                //   ┘     └
                //
                // Those count as 1
                //   ┌     ┐
                //   | and |
                //   ┘     └

                match pipe {
                    Pipe::NE => loop {
                        let p = points.next().unwrap();
                        match pipes.get(p).unwrap() {
                            Pipe::SE => {
                                // ┌
                                // |
                                // └
                                break;
                            }
                            Pipe::SW => {
                                // ┐
                                // |
                                // └
                                hit_outline += 1;
                                break;
                            }
                            _ => continue,
                        }
                    },
                    Pipe::NW => loop {
                        let p = points.next().unwrap();
                        match pipes.get(p).unwrap() {
                            Pipe::SW => {
                                // ┐
                                // |
                                // ┘
                                break;
                            }
                            Pipe::SE => {
                                // ┌
                                // |
                                // ┘
                                hit_outline += 1;
                                break;
                            }
                            _ => continue,
                        }
                    },

                    _ => {
                        hit_outline += 1;
                    }
                }
            }

            if hit_outline.is_odd() {
                inside.insert(current_pos);
            }
        }
    }

    Some(Int::try_from(inside.len()).unwrap())
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 20,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 22,
        ));
        assert_eq!(result, Some(10));
    }
}
