use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

type Int = u32;

/* -------------------------------------------------------------------------- */

const NB_OF_CARD: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 0,
    K = 1,
    Q = 2,
    J = 3,
    T = 4,
    N9 = 5,
    N8 = 6,
    N7 = 7,
    N6 = 8,
    N5 = 9,
    N4 = 10,
    N3 = 11,
    N2 = 12,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_card(c: char) -> Card {
    use Card::*;
    match c {
        'A' => A,
        'K' => K,
        'Q' => Q,
        'J' => J,
        'T' => T,
        '9' => N9,
        '8' => N8,
        '7' => N7,
        '6' => N6,
        '5' => N5,
        '4' => N4,
        '3' => N3,
        '2' => N2,
        _ => unreachable!(),
    }
}

fn parse_hand(s: &str) -> Hand {
    Hand(
        s.chars()
            .map(parse_card)
            .collect_vec()
            .try_into()
            .ok()
            .unwrap(),
    )
}

fn parse_input(input: &str) -> impl Iterator<Item = (Hand, Int)> + '_ {
    input.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();

        let hand = parse_hand(hand);
        let bid = bid.parse::<Int>().unwrap();

        (hand, bid)
    })
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<Int> {
    fn analyze_hand(hand: &Hand) -> HandType {
        let mut counts: [_; NB_OF_CARD] = [0; NB_OF_CARD];
        for card in &hand.0 {
            counts[*card as usize] += 1;
        }

        let mut has_triple: bool = false;
        let mut nb_of_pair = 0;

        for count in counts {
            match count {
                5 => return HandType::FiveOfKind,
                4 => return HandType::FourOfKind,
                3 => has_triple = true,
                2 => nb_of_pair += 1,
                _ => {}
            }
        }

        match (has_triple, nb_of_pair) {
            (true, 1) => HandType::FullHouse,
            (true, 0) => HandType::ThreeOfKind,
            (false, 2) => HandType::TwoPair,
            (false, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    let mut inputs = parse_input(input)
        .map(|(hand, bid)| {
            let hand_type = analyze_hand(&hand);
            ((hand_type, hand), bid)
        })
        .collect_vec();

    inputs.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let result = inputs
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, (_, bid))| (i as Int + 1) * bid)
        .sum();

    Some(result)
}

/* -------------------------------------------------------------------------- */

fn compare_card_with_joker(a: Card, b: Card) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if a == Card::J {
        // NOTE: ordering of card is lower is better
        Ordering::Greater
    } else if b == Card::J {
        // NOTE: ordering of card is lower is better
        Ordering::Less
    } else {
        a.cmp(&b)
    }
}

pub fn part_two(input: &str) -> Option<Int> {
    fn analyze_hand(hand: &Hand) -> HandType {
        let mut counts: [_; NB_OF_CARD] = [0; NB_OF_CARD];
        for &card in &hand.0 {
            counts[card as usize] += 1;
        }

        {
            let nb_of_j = counts[Card::J as usize];

            if nb_of_j != 0 {
                counts[Card::J as usize] = 0;
                let (mut the_max, rest) = counts.split_first_mut().unwrap();
                for x in rest {
                    if *x > *the_max {
                        the_max = x;
                    }
                }

                *the_max += nb_of_j;
            }
        }

        {
            let mut has_triple: bool = false;
            let mut nb_of_pair = 0;

            for count in counts {
                match count {
                    5 => return HandType::FiveOfKind,
                    4 => return HandType::FourOfKind,
                    3 => has_triple = true,
                    2 => nb_of_pair += 1,
                    _ => {}
                }
            }

            match (has_triple, nb_of_pair) {
                (true, 1) => HandType::FullHouse,
                (true, 0) => HandType::ThreeOfKind,
                (false, 2) => HandType::TwoPair,
                (false, 1) => HandType::OnePair,
                (false, 0) => HandType::HighCard,
                _ => unreachable!(),
            }
        }
    }

    let mut inputs = parse_input(input)
        .map(|(hand, bid)| (analyze_hand(&hand), hand, bid))
        .collect_vec();

    inputs.sort_unstable_by(|(a, hand_a, _), (b, hand_b, _)| match a.cmp(b) {
        Ordering::Equal => {
            for (a, b) in hand_a.0.iter().zip_eq(&hand_b.0) {
                match compare_card_with_joker(*a, *b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            Ordering::Equal
        }
        other => other,
    });

    let result = inputs
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as Int + 1) * bid)
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
