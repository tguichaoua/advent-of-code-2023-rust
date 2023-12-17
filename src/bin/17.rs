// TODO: refactor the code (someday)

use std::collections::HashMap;

use advent_of_code::helper::{
    array_2d::Array2D,
    carte::{Direction, Pos},
};
use priority_queue::PriorityQueue;

advent_of_code::solution!(17);

type Int = u32;

/* -------------------------------------------------------------------------- */

fn parse_input(input: &str) -> Array2D<u32> {
    Array2D::from_iter(
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let grid = parse_input(input);

    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    struct Node {
        heat: Int,
        forward: u8,
        position: Pos,
        direction: Direction,
    }

    #[derive(Hash, PartialEq, Eq)]
    struct HistoryKey {
        forward: u8,
        position: Pos,
        direction: Direction,
    }

    impl Node {
        fn history_key(&self) -> HistoryKey {
            HistoryKey {
                forward: self.forward,
                position: self.position,
                direction: self.direction,
            }
        }
    }

    let start_node = Node {
        heat: 0,
        forward: 0,
        position: Pos { x: 0, y: 0 },
        direction: Direction::Right,
    };

    let mut history = HashMap::new();
    history.insert(start_node.history_key(), start_node.heat);

    let mut queue = PriorityQueue::new();
    queue.push(start_node, 0);

    let width = grid.width();
    let height = grid.height();

    let end_pos = Pos {
        x: width - 1,
        y: height - 1,
    };

    let mut minimal_heat = Int::MAX;

    while let Some((
        Node {
            heat,
            forward,
            position,
            direction,
        },
        _,
    )) = queue.pop()
    {
        if position == end_pos {
            minimal_heat = minimal_heat.min(heat);

            continue;
        }

        // Can I got forward ?
        if forward < 3 {
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();
                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: forward + 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }

        // Can I turn right ?
        {
            let direction = direction.turn_right();
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();

                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }

        // Can I turn left ?
        {
            let direction = direction.turn_left();
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();
                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }
    }

    Some(minimal_heat)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<Int> {
    let grid = parse_input(input);

    #[derive(Hash, Eq, PartialEq, Clone, Copy)]
    struct Node {
        heat: Int,
        forward: u8,
        position: Pos,
        direction: Direction,
    }

    #[derive(Hash, PartialEq, Eq)]
    struct HistoryKey {
        forward: u8,
        position: Pos,
        direction: Direction,
    }

    impl Node {
        fn history_key(&self) -> HistoryKey {
            HistoryKey {
                forward: self.forward,
                position: self.position,
                direction: self.direction,
            }
        }
    }

    let start_node = Node {
        heat: 0,
        forward: 0,
        position: Pos { x: 0, y: 0 },
        direction: Direction::Right,
    };

    let mut history = HashMap::new();
    history.insert(start_node.history_key(), start_node.heat);

    let mut queue = PriorityQueue::new();
    queue.push(start_node, 0);

    let width = grid.width();
    let height = grid.height();

    let end_pos = Pos {
        x: width - 1,
        y: height - 1,
    };

    let mut minimal_heat = Int::MAX;

    while let Some((
        Node {
            heat,
            forward,
            position,
            direction,
        },
        _,
    )) = queue.pop()
    {
        if position == end_pos {
            if forward >= 4 {
                minimal_heat = minimal_heat.min(heat);
            }

            continue;
        }

        // Can I got forward ?
        if forward < 10 {
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();
                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: forward + 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }

        // Can I turn right ?
        if forward >= 4 {
            let direction = direction.turn_right();
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();

                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }

        // Can I turn left ?
        if forward >= 4 {
            let direction = direction.turn_left();
            if let Some(position) = position.move_to_clamped(direction, width, height) {
                let heat = heat + grid.get(position.x, position.y).unwrap();
                if heat < minimal_heat {
                    let priority = Int::MAX - heat;

                    let node = Node {
                        heat,
                        forward: 1,
                        position,
                        direction,
                    };

                    let history_heat = history.entry(node.history_key()).or_insert(Int::MAX);

                    if heat < *history_heat {
                        *history_heat = heat;
                        queue.push(node, priority);
                    }
                }
            }
        }
    }

    Some(minimal_heat)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(71));
    }
}
