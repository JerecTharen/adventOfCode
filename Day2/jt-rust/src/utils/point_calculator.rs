pub enum Outcomes{
    WIN,
    LOSE,
    DRAW
}

#[derive(PartialEq)]
pub enum PlayOptions{
    ROCK,
    PAPER,
    SCISSORS
}

pub fn get_outcome_points(outcome: Outcomes)->u32{
    match outcome{
        Outcomes::WIN => 6,
        Outcomes::DRAW => 3,
        Outcomes::LOSE => 0
    }
}

pub fn get_play_option_points(play_option: PlayOptions)->u32{
    match play_option{
        PlayOptions::ROCK => 1,
        PlayOptions::PAPER => 2,
        PlayOptions::SCISSORS => 3
    }
}

pub fn get_outcome_from_play_options(player_option: &PlayOptions, opponent_option: &PlayOptions)->Outcomes{
    if player_option == opponent_option{
        return Outcomes::DRAW;
    }
    else if player_option == &PlayOptions::ROCK && opponent_option == &PlayOptions::SCISSORS{
        return  Outcomes::WIN;
    }
    else if player_option == &PlayOptions::ROCK && opponent_option == &PlayOptions::PAPER{
        return  Outcomes::LOSE;
    }
    else if player_option == &PlayOptions::PAPER && opponent_option == &PlayOptions::ROCK{
        return  Outcomes::WIN;
    }
    else if player_option == &PlayOptions::PAPER && opponent_option == &PlayOptions::SCISSORS{
        return  Outcomes::LOSE;
    }
    else if player_option == &PlayOptions::SCISSORS && opponent_option == &PlayOptions::PAPER{
        return  Outcomes::WIN;
    }
    else{
        return Outcomes::LOSE;
    }
}

pub fn get_round_points(player_option: PlayOptions, opponent_option: PlayOptions)->u32{
    let outcome = get_outcome_from_play_options(&player_option, &opponent_option);
    return get_play_option_points(player_option) + get_outcome_points(outcome);
}