use std::time::Instant;

fn main() {
    let input = include_str!("input");

    let start_time = Instant::now();
    println!("P1: {}", get_value(input, false));
    println!("P2: {}", get_value(input, true));
    println!("Took {} secs", start_time.elapsed().as_secs_f32());
}


fn get_value(input: &str, str_vals: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            if str_vals {
                line
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vals| vals.first().unwrap() * 10 + vals.last().unwrap())
        .sum()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX1: &str = include_str!("example1");
    const EX2: &str = include_str!("example2");

    #[test]
    fn test1() {
        assert_eq!(get_value(EX1, false), 142);
    }

    #[test]
    fn test2() {
        assert_eq!(get_value(EX2, true), 281);
    }

}