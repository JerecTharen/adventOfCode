use crate::utils::tree_counter::{count_visible_trees};

#[test]
fn test_count_visible_trees_should_do_example_correctly(){
    const EXPECTED_COUNT: u32 = 21;
    let example_input = "30373
25512
65332
33549
35390".to_string();

//Final should be 003, 010, 023, 037, 043,
//102, 115, 125, 142,
//206, 233, 242,
//303, 325, 349,
//403, 415, 439, 440
    let actual_count = count_visible_trees(example_input, true);
    assert_eq!(actual_count, EXPECTED_COUNT);
}

#[test]
fn test_count_visible_trees_should_filter_empty_lines(){
    const EXPECTED_COUNT: u32 = 21;
    let example_input = "
30373
25512
65332
33549
35390
".to_string();

//Final should be 003, 010, 023, 037, 043,
//102, 115, 125, 142,
//206, 233, 242,
//303, 325, 349,
//403, 415, 439, 440
    let actual_count = count_visible_trees(example_input, true);
    assert_eq!(actual_count, EXPECTED_COUNT);
}