use std::time::Instant;
use std::collections::HashSet;

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


fn parse_input(input: &str) -> Schematic {
    Schematic::parse(input)
}


fn p1(schematic: &Schematic) -> String {
    format!("{}", get_pn_sum(schematic))
}


fn p2(schematic: &Schematic) -> String {
    format!("{}", get_gear_ratio_sum(schematic))
}


#[derive(Copy, Clone)]
enum Part {
    NumberIdx(usize),
    Symbol(char),
    Empty
}


struct Schematic {
    data: Vec<Part>,
    part_numbers: Vec<u32>,
    width: usize,
}

impl Schematic {
    pub fn parse(input: &str) -> Self {
        let mut data = vec![];
        let mut part_numbers = vec![];
        let width = input.lines().next().unwrap().len();

        let mut cur_val: Option<u32> = None;
        for line in input.lines() {
            for c in line.chars() {
                match c {
                    '.' => data.push(Part::Empty),
                    '0'..='9' => data.push(Part::NumberIdx(part_numbers.len())),
                    _ => data.push(Part::Symbol(c))
                }
                if let Some(val) = cur_val {
                    if c.is_ascii_digit() {
                        cur_val = Some(val * 10 + c.to_digit(10).unwrap());
                    } else {
                        part_numbers.push(val);
                        cur_val = None;
                    }
                } else if c.is_ascii_digit() {
                    cur_val = Some(c.to_digit(10).unwrap());
                }
            }
        }

        Self {
            data,
            part_numbers,
            width
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Part> {
        let idx = y * self.width + x;
        self.data.get(idx)
    }

    pub fn get_pn(&self, idx: usize) -> u32 {
        *self.part_numbers.get(idx).unwrap()
    }
}


fn get_pn_sum(schematic: &Schematic) -> u32 {
    let mut adj_partnum_idxs = HashSet::new();

    for y in 0..(schematic.data.len()/schematic.width) {
        for x in 0..schematic.width {
            if matches!(schematic.get(x, y).unwrap(), Part::Symbol(_)) {
                for x1 in (x.saturating_sub(1))..=(x+1) {
                    for y1 in (y.saturating_sub(1))..=(y+1) {
                        if let Some(Part::NumberIdx(idx)) = schematic.get(x1, y1) {
                            adj_partnum_idxs.insert(*idx);
                        }
                    }
                }
            }
        }
    }

    adj_partnum_idxs.iter()
        .map(|&idx| schematic.get_pn(idx))
        .sum()
}


fn get_gear_ratio_sum(schematic: &Schematic) -> u32 {
    let mut gear_sum = 0;

    for y in 0..(schematic.data.len()/schematic.width) {
        for x in 0..schematic.width {
            if matches!(schematic.get(x, y).unwrap(), Part::Symbol('*')) {
                let mut adj_partnum_idxs = HashSet::new();
                for x1 in (x.saturating_sub(1))..=(x+1) {
                    for y1 in (y.saturating_sub(1))..=(y+1) {
                        if let Some(Part::NumberIdx(idx)) = schematic.get(x1, y1) {
                            adj_partnum_idxs.insert(*idx);
                        }
                    }
                }
                if adj_partnum_idxs.len() == 2 {
                    let gear_ratio: u32 = adj_partnum_idxs.iter()
                        .map(|&idx| schematic.get_pn(idx))
                        .product();
                    gear_sum += gear_ratio;
                }
            }
        }
    }

    gear_sum
}


#[cfg(test)]
mod test {
    use super::*;

    const EX1: &str = include_str!("example1");

    #[test]
    fn test1() {
        assert_eq!(get_pn_sum(&parse_input(EX1)), 4361)
    }

    #[test]
    fn test2() {
        assert_eq!(get_gear_ratio_sum(&parse_input(EX1)), 467835)
    }

}