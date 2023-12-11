use itertools::Itertools;

advent_of_code::solution!(6);

/* -------------------------------------------------------------------------- */

fn calculate_nb_solution_for_race(time: u64, distance: u64) -> u64 {
    #![allow(clippy::cast_possible_truncation)]

    // Let `d` the distance the boat travel
    // Let `D` the distance to beat
    // Let `t` the time spent to press the button
    // Let `T` the total time for the race
    // Let `v` the speed of the boat

    // The speed of the boat increase by 1 mm/ms for each ms the button is pressed:
    // v = t . 1

    // We want
    //            d > D
    //      v.(T-t) > D
    //      t.(T-t) > D
    //      tT - t² > D
    // -t² + tT - D > 0

    // We have a polynomial aX² + bX + c
    // with a = -1
    //      b = T
    //      c = -D

    // Since a < 0, the polynomial is strictly positive for t_1 < t < t_2
    // with t_1 and t_2 the roots of the polynomial.

    // Let's calculate the roots of -t² + Tt - D

    // delta = b² - 4ac
    //       = T² - 4.(-1).(-D)
    //       = T² - 4.D
    let Some(delta) = (time * time).checked_sub(4 * distance) else {
        // `delta` is negative, there is no real solution
        return 0;
    };

    let sqrt_delta = f64::sqrt(delta as f64);

    // t_1,2 = ( -b ± sqrt(delta) ) / ( 2.a )
    //       = ( -T ± sqrt(delta) ) / ( 2.(-1) )
    //       = (  T ± sqrt(delta) ) / ( 2 )
    let t_1 = (time as f64 - sqrt_delta) / 2.0;
    let t_2 = (time as f64 + sqrt_delta) / 2.0;

    // Since we can only press the button for an integer amount of time
    // we need to round t_1 and t_2.

    let t_1 = if t_1 < 0.0 {
        // time cannot be negative
        0
    } else {
        // We want the integer greater than t_1 but not t_1 itself (if it's an integer),
        // because the inequality above is strict.

        // t_1 is not negative
        #[allow(clippy::cast_sign_loss)]
        {
            (t_1 + 1.0).floor() as u64
        }
    };

    let t_2 = {
        // Same as of t_1, we want the integer lower than t_2 but not t_2 itself.

        // t_2 cannot be negative
        #[allow(clippy::cast_sign_loss)]
        {
            (t_2 - 1.0).ceil() as u64
        }
    };

    // The number of solution is the number of integer value between t_1 and t_2.
    t_2 - t_1 + 1
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());

    let distances = distances
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());

    let result = times
        .zip_eq(distances)
        .map(|(time, distance)| calculate_nb_solution_for_race(time, distance))
        .product1()
        .unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

pub fn part_two(input: &str) -> Option<u64> {
    let (times, distances) = input.split_once('\n').unwrap();

    let time = times
        .strip_prefix("Time:")
        .unwrap()
        .replace(|c: char| c.is_whitespace(), "")
        .parse::<u64>()
        .unwrap();

    let distance = distances
        .strip_prefix("Distance:")
        .unwrap()
        .replace(|c: char| c.is_whitespace(), "")
        .parse::<u64>()
        .unwrap();

    let result = calculate_nb_solution_for_race(time, distance);

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
