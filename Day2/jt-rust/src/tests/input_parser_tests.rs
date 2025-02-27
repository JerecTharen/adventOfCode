use crate::utils::point_calculator::{PlayOptions, Outcomes};
use crate::utils::input_parser::{parse_first_character, parse_second_character, part_2_parse_second_character};

#[test]
fn test_parse_first_character_a_is_rock(){
    let stub_char = 'A';

    let actual_play_option = parse_first_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::ROCK));
}
#[test]
fn test_parse_first_character_b_is_paper(){
    let stub_char = 'B';

    let actual_play_option = parse_first_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::PAPER));
}
#[test]
fn test_parse_first_character_c_is_scissors(){
    let stub_char = 'C';

    let actual_play_option = parse_first_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::SCISSORS));
}

#[test]
fn test_parse_second_character_y_is_paper(){
    let stub_char = 'Y';

    let actual_play_option = parse_second_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::PAPER));
}
#[test]
fn test_parse_second_character_x_is_rock(){
    let stub_char = 'X';

    let actual_play_option = parse_second_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::ROCK));
}
#[test]
fn test_parse_second_character_z_is_scissors(){
    let stub_char = 'Z';

    let actual_play_option = parse_second_character(stub_char);

    assert!(matches!(actual_play_option, PlayOptions::SCISSORS));
}

#[test]
fn test_part_2_parse_second_character_x_is_lose(){
    let actual_outcome = part_2_parse_second_character('X');

    assert!(matches!(actual_outcome, Outcomes::LOSE));
}
#[test]
fn test_part_2_parse_second_character_y_is_draw(){
    let actual_outcome = part_2_parse_second_character('Y');

    assert!(matches!(actual_outcome, Outcomes::DRAW));
}
#[test]
fn test_part_2_parse_second_character_z_is_win(){
    let actual_outcome = part_2_parse_second_character('Z');

    assert!(matches!(actual_outcome, Outcomes::WIN));
}