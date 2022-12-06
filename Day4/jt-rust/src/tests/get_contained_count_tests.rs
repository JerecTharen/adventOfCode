use crate::utils::get_contained_count;

#[test]
fn test_get_contained_count_finds_two_complete_overlap_in_example_input(){
    let expected_count = 2_u32;
    let mut actual_count = 0_u32;
    let is_any_overlap_counted = false;
    let test_input: String = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

    get_contained_count(test_input, &mut actual_count, is_any_overlap_counted);

    assert_eq!(actual_count, expected_count);
}
#[test]
fn test_get_contained_count_finds_four_any_overlap_in_example_input(){
    let expected_count = 4_u32;
    let mut actual_count = 0_u32;
    let is_any_overlap_counted = true;
    let test_input: String = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

    get_contained_count(test_input, &mut actual_count, is_any_overlap_counted);

    assert_eq!(actual_count, expected_count);
}
#[test]
fn test_get_contained_count_finds_three_any_overlap_in_modified_example_input(){
    let expected_count = 3_u32;
    let mut actual_count = 0_u32;
    let is_any_overlap_counted = true;
    let test_input: String = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,7-8".to_string();

    get_contained_count(test_input, &mut actual_count, is_any_overlap_counted);

    assert_eq!(actual_count, expected_count);
}