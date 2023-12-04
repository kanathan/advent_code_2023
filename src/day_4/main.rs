use std::time::Instant;
use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, preceded, pair, tuple},
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


fn p1(input: &[Card]) -> String {
    format!("{}", input.iter().map(|card| card.get_points()).sum::<usize>())
}


fn p2(input: &[Card]) -> String {
    format!("{}", get_total_cards(&input))
}


struct Card {
    matches: usize,
}


impl Card {
    pub fn new(winning_numbers: Vec<usize>, picked_numbers: Vec<usize>) -> Self {
        let winning_set: HashSet<usize> = HashSet::from_iter(winning_numbers.into_iter());
        let picked_set: HashSet<usize> = HashSet::from_iter(picked_numbers.into_iter());
        let matches = winning_set.intersection(&picked_set).count();

        Self { matches }
    }

    pub fn get_points(&self) -> usize {
        if self.matches > 0 {
            usize::pow(2, (self.matches - 1) as u32)
        } else {
            0
        }
    }
}


fn get_total_cards(cards: &[Card]) -> u32 {
    let mut card_count = vec![1_u32; cards.len()];

    for idx in 0..cards.len() {
        let cur_card_matches = cards[idx].matches;
        for j in (idx+1)..=(idx+cur_card_matches) {
            card_count[j] += card_count[idx];
        }
    }

    card_count.into_iter().sum()
}


fn parse_input(input: &str) -> Vec<Card> {
    input.lines()
        .map(parse_line)
        .collect()
}


fn parse_line(input: &str) -> Card {
    let (_id, (winning_numbers, picked_numbers)) = separated_pair(
        parse_card_id, 
        pair(tag(":"), multispace1), 
        parse_number_lists)
        (input)
        .finish().unwrap().1;


    Card::new(winning_numbers, picked_numbers)
}


fn parse_number_lists(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    separated_pair(
        separated_list1(multispace1, parse_number), 
        tuple((multispace0, tag("|"), multispace0)), 
        separated_list1(multispace1, parse_number)
    )(input)
}


fn parse_card_id(input: &str) -> IResult<&str, usize> {
    preceded(pair(tag("Card"), multispace1), parse_number)(input)
}


fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}



#[cfg(test)]
mod test {
    use super::*;

    const EX1: &str = include_str!("example1");

    #[test]
    fn test1() {
        let cards = parse_input(EX1);

        assert_eq!(cards.iter().map(|card| card.get_points()).sum::<usize>(), 13)
    }

    #[test]
    fn test2() {
        let cards = parse_input(EX1);

        assert_eq!(get_total_cards(&cards), 30)
    }

}