advent_of_code::solution!(2);

/* -------------------------------------------------------------------------- */

struct Game {
    id: u32,
    draw: Vec<Draw>,
}

#[derive(Default, Clone, Copy)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (head, data) = line.split_once(':').unwrap();

            let id = head.strip_prefix("Game ").unwrap().parse().unwrap();

            let draw = data
                .split(';')
                .map(|draw| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    draw.split(',').for_each(|draw| {
                        let (count, color) = draw.trim().split_once(' ').unwrap();
                        let count: u32 = count.parse().unwrap();
                        match color {
                            "red" => red += count,
                            "green" => green += count,
                            "blue" => blue += count,
                            _ => unreachable!(),
                        }
                    });

                    Draw { red, green, blue }
                })
                .collect();

            Game { id, draw }
        })
        .collect()
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);

    let result = games
        .into_iter()
        .filter(|game| {
            game.draw
                .iter()
                .copied()
                .all(|Draw { red, green, blue }| red <= 12 && green <= 13 && blue <= 14)
        })
        .map(|game| game.id)
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);

    let result = games
        .into_iter()
        .map(|game| {
            let minimum_colors = game
                .draw
                .into_iter()
                .reduce(|acc, e| Draw {
                    red: acc.red.max(e.red),
                    green: acc.green.max(e.green),
                    blue: acc.blue.max(e.blue),
                })
                .unwrap_or_default();

            minimum_colors.red * minimum_colors.green * minimum_colors.blue
        })
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
