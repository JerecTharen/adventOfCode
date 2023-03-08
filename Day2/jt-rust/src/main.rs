use std::fs::read_to_string;
use crate::utils::{input_parser::{parse_first_character, parse_second_character}, point_calculator::get_round_points};

mod utils;
mod tests;

fn main() {
    println!("Hello There!");
    // let file_input = read_to_string("./ignore_data/example.txt").unwrap();
    let file_input = read_to_string("./ignore_data/input.txt").unwrap();
    let rounds = file_input.split("\n");
    let point_total:u32 = rounds.map(|round| {
        let mut played_option_chars = round.chars().filter(|c| c != &' ');
        let opponent_option = parse_first_character(played_option_chars.next().unwrap());
        let player_option = parse_second_character(played_option_chars.next().unwrap());

        get_round_points(player_option, opponent_option)
    }).sum();

    print!("Game total is {0}\n", point_total);
}
