use super::point_calculator::{PlayOptions, Outcomes};

pub fn parse_first_character(c: char)->PlayOptions{
    match c{
        'A' => PlayOptions::ROCK,
        'B' => PlayOptions::PAPER,
        'C' => PlayOptions::SCISSORS,
        _ => panic!("Argument out of range")
    }
}
pub fn parse_second_character(c: char)->PlayOptions{
    match c{
        'X' => PlayOptions::ROCK,
        'Y' => PlayOptions::PAPER,
        'Z' => PlayOptions::SCISSORS,
        _ => panic!("Argument out of range")
    }
}
pub fn part_2_parse_second_character(c: char)->Outcomes{
    match c{
        'X'=> Outcomes::LOSE,
        'Y'=> Outcomes::DRAW,
        'Z'=> Outcomes::WIN,
        _ => panic!("Argument out of range")
    }
}