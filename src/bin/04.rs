use itertools::Itertools;

advent_of_code::solution!(4);

/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy)]
struct Card {
    match_count: usize,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_head, rest) = line.split_once(':').unwrap();
            let (win_numbers, numbers) = rest.split_once('|').unwrap();

            let win_numbers = win_numbers
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();

            let numbers = numbers.split_ascii_whitespace().map(|x| x.parse().unwrap());

            let match_count = numbers.filter(|nb| win_numbers.contains(nb)).count();

            Card { match_count }
        })
        .collect()
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    let result = cards
        .into_iter()
        .map(|card| card.match_count.checked_sub(1).map(|x| 1 << x).unwrap_or(0))
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    let mut card_count = vec![1; cards.len()];

    for (i, card) in cards.into_iter().enumerate() {
        let count = card_count[i];
        for j in 0..card.match_count {
            card_count[i + 1 + j] += count;
        }
    }

    let result = card_count.into_iter().sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
