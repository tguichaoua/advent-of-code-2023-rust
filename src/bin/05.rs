use itertools::Itertools;

advent_of_code::solution!(5);

/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy)]
struct MapRange {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapRange {
    fn src_contains(&self, value: u64) -> bool {
        value >= self.src_start && value < self.src_start + self.len
    }
}

struct ResourceMap {
    ranges: Vec<MapRange>,
}

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    len: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value < self.start + self.len
    }
}

impl ResourceMap {
    fn map(&self, value: u64) -> u64 {
        for range in self.ranges.iter().copied() {
            if range.src_contains(value) {
                return value - range.src_start + range.dst_start;
            }
        }
        value
    }

    fn map_range(&self, mut range: Range) -> Vec<Range> {
        // Okay, this function looks horrible and should probably be refactored,
        // but look bellow how the rest of code look elegant thanks to this function :)

        let mut ranges = Vec::new();

        for map_range in self.ranges.iter().copied() {
            if map_range.src_contains(range.start) {
                let start = range.start - map_range.src_start + map_range.dst_start;
                let map_range_rest_len = map_range.len - (range.start - map_range.src_start);

                if range.len <= map_range_rest_len {
                    let len = range.len;
                    ranges.push(Range { start, len });
                    return ranges;
                } else {
                    let len = map_range_rest_len;
                    ranges.push(Range { start, len });

                    range.start += len;
                    range.len -= len;
                }
            } else if range.contains(map_range.src_start) {
                let first = Range {
                    start: range.start,
                    len: map_range.src_start - range.start,
                };

                let source_range_rest_len = range.len - first.len;

                if source_range_rest_len <= map_range.len {
                    let middle = Range {
                        start: map_range.dst_start,
                        len: source_range_rest_len,
                    };

                    ranges.push(first);
                    ranges.push(middle);

                    return ranges;
                } else {
                    let middle = Range {
                        start: map_range.dst_start,
                        len: map_range.len,
                    };

                    ranges.push(first);
                    ranges.push(middle);

                    range.start += first.len + map_range.len;
                    range.len -= first.len + map_range.len;
                }
            }
        }

        ranges.push(range);

        ranges
    }
}

struct Maps {
    seed_to_soil: ResourceMap,
    soil_to_fertilizer: ResourceMap,
    fertilizer_to_water: ResourceMap,
    water_to_light: ResourceMap,
    light_to_temperature: ResourceMap,
    temperature_to_humidity: ResourceMap,
    humidity_to_location: ResourceMap,
}

fn parse_input(input: &str) -> (impl Iterator<Item = u64> + '_, Maps) {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let _ = lines.next(); // empty line

    macro_rules! parse_resource_map {
        (
            $( $name:ident => $header:expr; )*
        ) => {
            $(
                let header = lines.next();
                debug_assert_eq!(header, Some($header));
                let $name = {
                    let mut ranges = lines
                        .by_ref()
                        .take_while(|line| !line.is_empty())
                        .map(parse_map_range)
                        .collect::<Vec<_>>();
                    ranges.sort_by_key(|range| range.src_start);
                    ResourceMap { ranges }
                };
            )*
        };
    }

    parse_resource_map! {
        seed_to_soil            => "seed-to-soil map:";
        soil_to_fertilizer      => "soil-to-fertilizer map:";
        fertilizer_to_water     => "fertilizer-to-water map:";
        water_to_light          => "water-to-light map:";
        light_to_temperature    => "light-to-temperature map:";
        temperature_to_humidity => "temperature-to-humidity map:";
        humidity_to_location    => "humidity-to-location map:";
    };

    (
        seeds,
        Maps {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn parse_map_range(str: &str) -> MapRange {
    let (a, b) = str.split_once(' ').unwrap();
    let (b, c) = b.split_once(' ').unwrap();

    let dst_start = a.parse().unwrap();
    let src_start = b.parse().unwrap();
    let len = c.parse().unwrap();

    MapRange {
        dst_start,
        src_start,
        len,
    }
}

/* -------------------------------------------------------------------------- */

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);

    let locations = seeds
        .map(|seed| maps.seed_to_soil.map(seed))
        .map(|soil| maps.soil_to_fertilizer.map(soil))
        .map(|fertilizer| maps.fertilizer_to_water.map(fertilizer))
        .map(|water| maps.water_to_light.map(water))
        .map(|light| maps.light_to_temperature.map(light))
        .map(|temperature| maps.temperature_to_humidity.map(temperature))
        .map(|humidity| maps.humidity_to_location.map(humidity));

    let result = locations.min().unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

// pub fn part_two(input: &str) -> Option<u64> {
//     let (seeds, maps) = parse_input(input);

//     let seeds = seeds.tuples().map(|(start, len)| Range { start, len });

//     let locations = seeds
//         .flat_map(|seeds| maps.seed_to_soil.map_range(seeds))
//         .flat_map(|soils| maps.soil_to_fertilizer.map_range(soils))
//         .flat_map(|fertilizers| maps.fertilizer_to_water.map_range(fertilizers))
//         .flat_map(|waters| maps.water_to_light.map_range(waters))
//         .flat_map(|lights| maps.light_to_temperature.map_range(lights))
//         .flat_map(|temperatures| maps.temperature_to_humidity.map_range(temperatures))
//         .flat_map(|humidities| maps.humidity_to_location.map_range(humidities));

//     let result = locations
//         .map(|localizations| localizations.start)
//         .min()
//         .unwrap();

//     Some(result)
// }

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);

    let seeds = seeds.tuples().map(|(start, len)| Range { start, len });

    let locations = seeds
        .flat_map(|seeds| maps.seed_to_soil.map_range(seeds))
        .flat_map(|soils| maps.soil_to_fertilizer.map_range(soils))
        .flat_map(|fertilizers| maps.fertilizer_to_water.map_range(fertilizers))
        .flat_map(|waters| maps.water_to_light.map_range(waters))
        .flat_map(|lights| maps.light_to_temperature.map_range(lights))
        .flat_map(|temperatures| maps.temperature_to_humidity.map_range(temperatures))
        .map(|humidities| humidities.start)
        .map(|humidity| maps.humidity_to_location.map(humidity));

    let result = locations.min().unwrap();

    Some(result)
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
