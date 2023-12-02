use std::time::Instant;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, preceded},
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


const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;


fn p1(games: &[Game]) -> String {
    format!("{}", get_valid_ids(games).iter().sum::<u32>())
}


fn p2(games: &[Game]) -> String {
    format!("{}", get_game_powers(games).iter().sum::<u32>())
}


#[derive(Debug)]
struct Game {
    id: u32,
    samples: Vec<Sample>
}

#[derive(Debug)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}


fn get_valid_ids(games: &[Game]) -> Vec<u32> {
    games.iter()
        .filter(|&game| valid_game(game))
        .map(|game| game.id)
        .collect()
}


fn valid_game(game: &Game) -> bool {
    game.samples.iter()
        .all(|sample| {
            sample.red <= MAX_RED &&
            sample.green <= MAX_GREEN &&
            sample.blue <= MAX_BLUE
        })
}


fn get_game_powers(games: &[Game]) -> Vec<u32> {
    games.iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for sample in game.samples.iter() {
                red = red.max(sample.red);
                green = green.max(sample.green);
                blue = blue.max(sample.blue);
            }

            red * green * blue
        })
        .collect()
}


fn parse_input(input: &str) -> Vec<Game> {
    input.lines()
        .map(parse_line)
        .collect()
}


fn parse_line(input: &str) -> Game {
    let (id, samples) = separated_pair(
        parse_id,
        tag(": "),
        parse_samples
    )(input).finish().unwrap().1;

    Game {
        id,
        samples
    }
}


fn parse_id(input: &str) -> IResult<&str, u32> {
    preceded(
        tag("Game "), 
        map_res(digit1, |s: &str| s.parse::<u32>())
    )(input)
}


fn parse_samples(input: &str) -> IResult<&str, Vec<Sample>> {
    separated_list1(tag("; "), parse_sample)(input)
}

fn parse_sample(input: &str) -> IResult<&str, Sample> {
    let (remaining, sample_list) = separated_list1(
        tag(", "),
        separated_pair(map_res(digit1, |s: &str| s.parse::<u32>()), tag(" "), alpha1)
    )(input)?;

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for (count, color) in sample_list {
        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => panic!("Invalid color {color}")
        }
    }

    IResult::Ok((remaining, Sample { red, green, blue }))
}



#[cfg(test)]
mod test {
    use super::*;

    const EX1: &str = include_str!("example1");

    #[test]
    fn test1() {
        assert_eq!(get_valid_ids(&parse_input(EX1)).iter().sum::<u32>(), 8)
    }

}