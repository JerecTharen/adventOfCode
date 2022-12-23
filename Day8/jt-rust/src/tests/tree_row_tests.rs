use crate::utils::tree_row::get_visible_trees;


#[test]
fn test_get_visible_trees_should_get_all_trees_for_first_row(){
    let expected_visible_trees: Vec<u32> = [1_u32,3_u32,2_u32].to_vec();
    let row = "132";
    let row_coord = 0;
    let plot_len = 3;

    let actual_visible_vector = get_visible_trees(row, row_coord, plot_len);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}

#[test]
fn test_get_visible_trees_should_get_first_and_last_trees_every_row(){
    let expected_visible_trees: Vec<u32> = [1_u32,3_u32,2_u32].to_vec();
    let row = "132";
    let row_coord = 2;
    let plot_len = 3;

    let actual_visible_vector = get_visible_trees(row, row_coord, plot_len);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}

#[test]
fn test_get_visible_trees_should_get_be_able_to_get_trees_from_middle(){
    let expected_visible_trees: Vec<u32> = [1_u32,3_u32].to_vec();
    let row = "132";
    let row_coord = 1;
    let plot_len = 3;

    let actual_visible_vector = get_visible_trees(row, row_coord, plot_len);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}