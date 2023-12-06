use std::time::Instant;

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{digit1, multispace1, space1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{separated_pair, preceded, pair, tuple, terminated},
    IResult, Finish
};

fn main() {
    let input = include_str!("input");

    let start = Instant::now();
    let parsed_input = parse_input(input);
    let parse_duration = start.elapsed().as_secs_f32();
    println!("Parsing took {parse_duration} secs");

    println!();

    let start = Instant::now();
    let p1_answer = p1(&parsed_input);
    let p1_duration = start.elapsed().as_secs_f32();
    println!("P1: {p1_answer}");
    println!("Took {p1_duration} secs");

    println!();

    let start = Instant::now();
    let p2_answer = p2(&parsed_input);
    let p2_duration = start.elapsed().as_secs_f32();
    println!("P2: {p2_answer}");
    println!("Took {p2_duration} secs");
}


fn p1(input: &(Vec<usize>, Vec<Map>)) -> String {
    format!("{}", get_min_location(input))
}


fn p2(input: &(Vec<usize>, Vec<Map>)) -> String {
    format!("{}", get_lowest_loc(input))
}


fn get_min_location(input: &(Vec<usize>, Vec<Map>)) -> usize {
    let (seeds, maps) = input;

    seeds.iter()
        .map(|seed| map_to_loc(*seed, maps))
        .min()
        .unwrap()
}


// Brute force method. Not efficient
fn get_lowest_loc(input: &(Vec<usize>, Vec<Map>)) -> usize {
    let (seeds, maps) = input;

    let mut min_loc = usize::MAX;

    for seed in seeds[0]..(seeds[0]+seeds[1]) {
        min_loc = min_loc.min(map_to_loc(seed, maps));
    }
    for seed in seeds[2]..(seeds[2]+seeds[3]) {
        min_loc = min_loc.min(map_to_loc(seed, maps));
    }

    min_loc
}


#[derive(Debug)]
struct Map {
    _name: String,
    map_ranges: Vec<MapRange>
}

#[derive(Debug)]
struct MapRange {
    source_start: usize,
    dest_start: usize,
    length: usize,
}


fn map_to_loc(val: usize, maps: &[Map]) -> usize {
    let mut dest = val;
    for map in maps.iter() {
        dest = map_value(dest, map);
    }
    dest
}


fn map_value(val: usize, map: &Map) -> usize {
    for map_range in map.map_ranges.iter() {
        if val >= map_range.source_start && val < map_range.source_start + map_range.length {
            let offset = val - map_range.source_start;
            return map_range.dest_start + offset;
        }
    }
    val // else maps to same dest
}


fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    separated_pair(
        parse_seeds, 
        multispace1, 
        parse_maps)
        (input)
        .finish()
        .unwrap()
        .1
}


fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("seeds: "), 
        separated_list1(multispace1, parse_value)
    )(input)
}


fn parse_value(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}


fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(multispace1, parse_map)(input)
}


fn parse_map(input: &str) -> IResult<&str, Map> {
    map(
        pair(
            parse_map_name,
            separated_list1(line_ending, parse_map_range)
        ),
        |(map_name, map_ranges)| {
            Map { _name: map_name.to_string(), map_ranges }
        }
    )(input)
}


fn parse_map_name(input: &str) -> IResult<&str, &str> {
    terminated(take_till(char::is_whitespace), tag(" map:\n"))(input)
}


fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    map(
        tuple((parse_value, space1, parse_value, space1, parse_value)),
        |(v1, _, v2, _, v3)| {
            MapRange {
                source_start: v2,
                dest_start: v1,
                length: v3
            }
        }
    )(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX1: &str = include_str!("example1");

    #[test]
    fn test1() {
        let (seeds, maps) = parse_input(EX1);
        assert_eq!(map_value(53, &maps[0]), 55);
        assert_eq!(map_value(10, &maps[0]), 10);
        assert_eq!(map_value(seeds[0], &maps[0]), 81);
        assert_eq!(map_value(seeds[1], &maps[0]), 14);
        assert_eq!(map_value(seeds[2], &maps[0]), 57);
        assert_eq!(map_value(seeds[3], &maps[0]), 13);
    }

    #[test]
    fn test2() {
        let (seeds, maps) = parse_input(EX1);

        let locations = seeds
            .into_iter()
            .map(|seed| map_to_loc(seed, &maps))
            .collect::<Vec<usize>>();

        assert_eq!(
            locations,
            vec![82, 43, 86, 35]
        )
    }

    #[test]
    fn test3() {
        let input = parse_input(EX1);

        assert_eq!(get_lowest_loc(&input), 46);
    }

}