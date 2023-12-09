use itertools::Itertools;

advent_of_code::solution!(9);

type Int = i64;

/* -------------------------------------------------------------------------- */

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = Int> + '_> + '_ {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<Int>().unwrap()))
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    let lines = parse_input(input);

    let result = lines
        .map(|line| {
            let mut lasts_of_sequence = Vec::new();

            let mut sequence = line.collect_vec();
            lasts_of_sequence.push(sequence.last().copied().unwrap());

            loop {
                sequence = sequence
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();

                if sequence.iter().all(|&x| x == 0) {
                    return lasts_of_sequence.into_iter().sum::<Int>();
                } else {
                    lasts_of_sequence.push(sequence.last().copied().unwrap());
                }
            }
        })
        .sum::<Int>();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<Int> {
    let lines = parse_input(input);

    let result = lines
        .map(|line| {
            let mut firsts_of_sequence = Vec::new();

            let mut sequence = line.collect_vec();
            firsts_of_sequence.push(sequence.first().copied().unwrap());

            loop {
                sequence = sequence
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();

                if sequence.iter().all(|&x| x == 0) {
                    return firsts_of_sequence
                        .into_iter()
                        .rev()
                        .fold(0, |acc, cur| cur - acc);
                } else {
                    firsts_of_sequence.push(sequence.first().copied().unwrap());
                }
            }
        })
        .sum::<Int>();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
