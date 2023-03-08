#[path="../utils/mod.rs"]
mod utils;

pub mod get_outcome_points{
    use crate::utils::point_calculator::{Outcomes, get_outcome_points};

    #[test]
    fn test_get_outcome_points_gets_6_for_win(){
        const EXPECTED_POINTS: u32 = 6;
        let stub_outcome = Outcomes::WIN;
    
        let actual_points = get_outcome_points(stub_outcome);
    
        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_outcome_points_gets_3_for_draw(){
        const EXPECTED_POINTS: u32 = 3;
        let stub_outcome = Outcomes::DRAW;
    
        let actual_points = get_outcome_points(stub_outcome);
    
        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_outcome_points_gets_0_for_lose(){
        const EXPECTED_POINTS: u32 = 0;
        let stub_outcome = Outcomes::LOSE;
    
        let actual_points = get_outcome_points(stub_outcome);
    
        assert_eq!(actual_points, EXPECTED_POINTS);
    }
}

pub mod get_play_option_points{
    use crate::utils::point_calculator::{get_play_option_points, PlayOptions};

    #[test]
    fn test_get_play_option_points_should_be_1_for_rock(){
        const EXPECTED_POINTS: u32 = 1;
        let stub_play_option = PlayOptions::ROCK;

        let actual_points = get_play_option_points(stub_play_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_play_option_points_should_be_2_for_paper(){
        const EXPECTED_POINTS: u32 = 2;
        let stub_play_option = PlayOptions::PAPER;

        let actual_points = get_play_option_points(stub_play_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_play_option_points_should_be_3_for_scissors(){
        const EXPECTED_POINTS: u32 = 3;
        let stub_play_option = PlayOptions::SCISSORS;

        let actual_points = get_play_option_points(stub_play_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
}

pub mod get_outcome_from_play_options{
    use crate::utils::point_calculator::{Outcomes, PlayOptions, get_outcome_from_play_options};

    #[test]
    fn test_get_outcome_from_play_options_draw_for_rock(){
        let stub_player_option = PlayOptions::ROCK;
        let stub_oponent_option = PlayOptions::ROCK;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::DRAW));
    }
    #[test]
    fn test_get_outcome_from_play_options_win_for_rock(){
        let stub_player_option = PlayOptions::ROCK;
        let stub_oponent_option = PlayOptions::SCISSORS;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::WIN));
    }
    #[test]
    fn test_get_outcome_from_play_options_lose_for_rock(){
        let stub_player_option = PlayOptions::ROCK;
        let stub_oponent_option = PlayOptions::PAPER;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::LOSE));
    }
    #[test]
    fn test_get_outcome_from_play_options_draw_for_paper(){
        let stub_player_option = PlayOptions::PAPER;
        let stub_oponent_option = PlayOptions::PAPER;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::DRAW));
    }
    #[test]
    fn test_get_outcome_from_play_options_win_for_paper(){
        let stub_player_option = PlayOptions::PAPER;
        let stub_oponent_option = PlayOptions::ROCK;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::WIN));
    }
    #[test]
    fn test_get_outcome_from_play_options_lose_for_paper(){
        let stub_player_option = PlayOptions::PAPER;
        let stub_oponent_option = PlayOptions::SCISSORS;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::LOSE));
    }
    #[test]
    fn test_get_outcome_from_play_options_draw_for_scissors(){
        let stub_player_option = PlayOptions::SCISSORS;
        let stub_oponent_option = PlayOptions::SCISSORS;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::DRAW));
    }
    #[test]
    fn test_get_outcome_from_play_options_win_for_scissors(){
        let stub_player_option = PlayOptions::SCISSORS;
        let stub_oponent_option = PlayOptions::PAPER;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::WIN));
    }
    #[test]
    fn test_get_outcome_from_play_options_lose_for_scissors(){
        let stub_player_option = PlayOptions::SCISSORS;
        let stub_oponent_option = PlayOptions::ROCK;
        
        let actual_outcome = get_outcome_from_play_options(&stub_player_option, &stub_oponent_option);

        assert!(matches!(actual_outcome, Outcomes::LOSE));
    }
}

pub mod get_round_points{
    use crate::utils::point_calculator::{get_round_points, PlayOptions};

    #[test]
    fn test_get_round_points_paper_rock_is_8(){
        const EXPECTED_POINTS: u32 = 8;
        let stub_player_option = PlayOptions::PAPER;
        let sutb_opponent_option = PlayOptions::ROCK;

        let actual_points = get_round_points(stub_player_option, sutb_opponent_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_round_points_rock_paper_is_1(){
        const EXPECTED_POINTS: u32 = 1;
        let stub_player_option = PlayOptions::ROCK;
        let sutb_opponent_option = PlayOptions::PAPER;

        let actual_points = get_round_points(stub_player_option, sutb_opponent_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
    #[test]
    fn test_get_round_points_both_scissors_is_6(){
        const EXPECTED_POINTS: u32 = 6;
        let stub_player_option = PlayOptions::SCISSORS;
        let sutb_opponent_option = PlayOptions::SCISSORS;

        let actual_points = get_round_points(stub_player_option, sutb_opponent_option);

        assert_eq!(actual_points, EXPECTED_POINTS);
    }
}
