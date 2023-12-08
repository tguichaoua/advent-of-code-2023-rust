use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

type Int = u32;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Outcomes<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> Option<Int> {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let empty = lines.next();
    debug_assert!(empty.unwrap().is_empty());

    let paths = lines
        .map(|line| {
            let (name, outcomes) = line.split_once('=').unwrap();

            let name = name.trim();

            let (left, right) = outcomes
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(',')
                .unwrap();

            let left = left.trim();
            let right = right.trim();

            (name, Outcomes { left, right })
        })
        .collect::<HashMap<_, _>>();

    let mut location = "AAA";
    let mut instructions = instructions.iter().copied().cycle();
    let mut step = 0;
    loop {
        step += 1;
        let path = paths.get(location).unwrap();
        location = match instructions.next().unwrap() {
            Direction::Left => path.left,
            Direction::Right => path.right,
        };
        if location == "ZZZ" {
            break;
        }
    }

    Some(step)
}

pub fn part_two(input: &str) -> Option<Int> {
    None

    // let mut lines = input.lines();

    // let instructions = lines
    //     .next()
    //     .unwrap()
    //     .as_bytes()
    //     .iter()
    //     .map(|b| match b {
    //         b'L' => Direction::Left,
    //         b'R' => Direction::Right,
    //         _ => unreachable!(),
    //     })
    //     .collect_vec();

    // let empty = lines.next();
    // debug_assert!(empty.unwrap().is_empty());

    // let paths = lines
    //     .map(|line| {
    //         let (name, outcomes) = line.split_once('=').unwrap();

    //         let name = name.trim();

    //         let (left, right) = outcomes
    //             .trim()
    //             .strip_prefix('(')
    //             .unwrap()
    //             .strip_suffix(')')
    //             .unwrap()
    //             .split_once(',')
    //             .unwrap();

    //         let left = left.trim();
    //         let right = right.trim();

    //         (name, Outcomes { left, right })
    //     })
    //     .collect::<HashMap<_, _>>();

    // let start_locations = paths.keys().filter(|name| name.ends_with('A')).copied();

    // #[derive(Debug)]
    // struct Loop {
    //     step_where_z: Int,
    //     len: Int,
    // }

    // let loops = start_locations
    //     .map(|mut location| {
    //         let mut states = HashSet::new();
    //         let mut step = 0;
    //         let mut step_where_z = Vec::new();
    //         let mut instructions = instructions.clone().into_iter().enumerate().cycle();

    //         loop {
    //             let (i, instruction) = instructions.next().unwrap();
    //             if !states.insert((location, i)) {
    //                 break;
    //             }

    //             step += 1;
    //             let path = paths.get(location).unwrap();
    //             location = match instruction {
    //                 Direction::Left => path.left,
    //                 Direction::Right => path.right,
    //             };

    //             if location.ends_with('Z') {
    //                 step_where_z.push(step);
    //             }
    //         }

    //         // NOTE: in input, each loop contains only one exit
    //         let step_where_z = *step_where_z.first().unwrap();

    //         Loop {
    //             step_where_z,
    //             len: states.len() as Int,
    //         }
    //     })
    //     .collect_vec();
    // .reduce(
    //     |Loop {
    //          step_where_z: mut b,
    //          len: mut a,
    //      },
    //      Loop {
    //          step_where_z: mut d,
    //          len: mut c,
    //      }| {
    //         if b < d {
    //             mem::swap(&mut b, &mut d);
    //             mem::swap(&mut a, &mut c);
    //         }

    //         for x in 0.. {
    //             let num = a * x + b - d;

    //             if num % c == 0 {
    //                 let y = num / c;

    //                 if a * (x + 1) == c * (y + 1) {
    //                     let step_where_z = a * x + b;

    //                     let len = (a * c) / (gcd::Gcd::gcd(a, c));

    //                     return Loop { step_where_z, len };
    //                 }
    //             }
    //         }
    //         unreachable!()
    //     },
    // )
    // .unwrap();

    // let mut x = 0;
    // let (
    //     Loop {
    //         step_where_z: b,
    //         len: a,
    //     },
    //     loops,
    // ) = loops.split_first().unwrap();
    // let step = 'outer: loop {
    //     x += 1;

    //     for Loop {
    //         step_where_z: d,
    //         len: c,
    //     } in loops
    //     {
    //         let num = a * x + b - d;
    //         if num % c != 0 {
    //             continue 'outer;
    //         }
    //     }

    //     break a * x + b;
    // };

    // Some(step)

    // let mut x = 0;
    // let step = loop {
    //     x += 1;
    //     let is_valid = rest.iter().all(
    //         |Loop {
    //              step_where_z: d,
    //              len: c,
    //          }| { ((d - b + a * x) % c) == 0 },
    //     );
    //     if !is_valid {
    //         continue;
    //     }

    //     let Ok(step) = rest
    //         .iter()
    //         .map(
    //             |Loop {
    //                  step_where_z: d,
    //                  len: c,
    //              }| { (d - b + a * x) / c },
    //         )
    //         .all_equal_value()
    //     else {
    //         continue;
    //     };

    //     break step;
    // };

    // Some(step as Int)
}

// pub fn part_two(input: &str) -> Option<Int> {
//     let mut lines = input.lines();

//     let instructions = lines
//         .next()
//         .unwrap()
//         .as_bytes()
//         .iter()
//         .map(|b| match b {
//             b'L' => Direction::Left,
//             b'R' => Direction::Right,
//             _ => unreachable!(),
//         })
//         .collect_vec();

//     let empty = lines.next();
//     debug_assert!(empty.unwrap().is_empty());

//     let paths = lines
//         .map(|line| {
//             let (name, outcomes) = line.split_once('=').unwrap();

//             let name = name.trim();

//             let (left, right) = outcomes
//                 .trim()
//                 .strip_prefix('(')
//                 .unwrap()
//                 .strip_suffix(')')
//                 .unwrap()
//                 .split_once(',')
//                 .unwrap();

//             let left = left.trim();
//             let right = right.trim();

//             (name, Outcomes { left, right })
//         })
//         .collect::<HashMap<_, _>>();

//     let locations = paths.keys().filter(|name| name.ends_with('A')).copied();

//     let result = thread::scope(|scope| {
//         let ghosts = locations
//             .map(|location| {
//                 let (tx, rx) = mpsc::sync_channel(1024);
//                 let paths = &paths;
//                 let instructions = &instructions;
//                 scope.spawn(move || {
//                     let mut location = location;
//                     let mut instructions = instructions.iter().copied().cycle();
//                     loop {
//                         let path = paths.get(location).unwrap();
//                         location = match instructions.next().unwrap() {
//                             Direction::Left => path.left,
//                             Direction::Right => path.right,
//                         };
//                         if tx.send(location.ends_with('Z')).is_err() {
//                             break;
//                         }
//                     }
//                 });

//                 rx
//             })
//             .collect_vec();

//         let mut step = 0;
//         loop {
//             step += 1;
//             let all_reach_end = ghosts.iter().map(|rx| rx.recv().unwrap()).all(|x| x);
//             let all_reach_end = ghosts
//                 .iter()
//                 .map(|rx| rx.recv().unwrap())
//                 .filter(|x| *x)
//                 .count();

//             if all_reach_end > 3 {
//                 println!("{all_reach_end}");
//             }
//             // if all_reach_end {
//             //     break;
//             // }
//         }

//         step
//     });

//     Some(result)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
