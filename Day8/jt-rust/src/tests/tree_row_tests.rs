use crate::utils::tree_row::{get_visible_trees, get_verticle_rows, create_tree_coord};


#[test]
fn test_get_visible_trees_should_get_all_trees_for_first_row(){
    let expected_visible_trees: Vec<&str> = ["001", "013", "022"].to_vec();
    let row = "132";
    let row_coord = 0;
    let plot_len = 3;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, false, false);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}
#[test]
fn test_get_visible_trees_should_get_all_trees_last_row(){
    let expected_visible_trees: Vec<&str> = ["201", "213", "222"].to_vec();
    let row = "132";
    let row_coord = 2;
    let plot_len = 3;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, false, false);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}
#[test]
fn test_get_visible_trees_should_count_backwards_for_reverse_row(){
    let expected_visible_trees: Vec<&str> = ["022", "013", "001"].to_vec();
    let row = "231";
    let row_coord = 0;
    let plot_len = 3;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, true, false);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}
#[test]
fn test_get_visible_trees_should_count_backwards_for_reverse_row_for_third_row_example(){
    let expected_visible_trees: Vec<&str> = ["242", "233", "215", "206"].to_vec();
    let row = "23356";
    let row_coord = 2;
    let plot_len = 5;
    let stub_is_reverse = true;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, stub_is_reverse, false);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}
#[test]
fn test_get_visible_trees_should_be_able_to_do_verticle_rows(){
    let expected_visible_trees: Vec<&str> = ["012", "113", "315", "416"].to_vec();
    let row = "23356";
    let row_coord = 1;
    let plot_len = 5;
    let stub_is_reverse = false;
    let stub_is_verticle = true;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, stub_is_reverse, stub_is_verticle);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}

#[test]
fn test_get_visible_trees_should_get_be_able_to_get_trees_from_middle(){
    let expected_visible_trees: Vec<&str> = ["101", "113"].to_vec();
    let row = "132";
    let row_coord = 1;
    let plot_len = 3;

    let actual_visible_vector = 
        get_visible_trees(row, row_coord, plot_len, false, false);

    assert_eq!(actual_visible_vector, expected_visible_trees);
}

#[test]
fn test_get_verticle_rows(){
    let expected_visible_trees: Vec<&str> = ["123", "456", "789"].to_vec();
    let tree_rows_start = ["147", "258", "369"].to_vec();

    let actual_verticle_rows = get_verticle_rows(tree_rows_start);

    assert_eq!(actual_verticle_rows, expected_visible_trees);
}

//This example comes from a coordinate that was being created incorreclty
#[test]
fn test_create_tree_coord_should_do_row_col_size_for_206(){
    const EXPECTED_TREE_COORD: &str = "206";
    let stub_row_coord = 2;
    let stub_column_num = 0;
    let stub_size = 6;


    let actual_tree_coord = create_tree_coord(stub_row_coord, stub_column_num, stub_size);

    assert_eq!(actual_tree_coord, EXPECTED_TREE_COORD);
}