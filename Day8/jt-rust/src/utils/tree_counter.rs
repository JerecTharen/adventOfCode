use std::collections::HashSet;

use super::tree_row::{get_visible_trees, get_verticle_rows};

pub fn count_visible_trees(input_grid: String, is_debug: bool) -> u32{
    let horizontal_tree_rows: Vec<&str> = input_grid
        .split("\n")
        .into_iter()
        .filter(|line| line.trim() != "")
        .collect();
    let plot_len: u32 = horizontal_tree_rows.len().try_into().unwrap();
    let mut visible_trees: Vec<String> = Vec::new();

    //Get all the visible trees from a line as coordinates (both forward and reverse)
    let mut row_coord = 0;
    //Have to do this slicing so that the vec isn't borrowed and unusable after the loop
    let horizontal_slice = &horizontal_tree_rows[..];
    for horizontal_row in horizontal_slice.iter().clone(){
        visible_trees
            .append(&mut get_visible_trees(
                horizontal_row, 
                row_coord, 
                plot_len, 
                false,
                false
            ));
        let reverse_row: &str = &horizontal_row.chars().rev().collect::<String>();
        visible_trees
            .append(&mut get_visible_trees(
                reverse_row, 
                row_coord, 
                plot_len, 
                true,
            false
        ));
        row_coord = row_coord + 1;
    }

    //Create lines for each possible direction
    let verticle_rows = get_verticle_rows(horizontal_tree_rows);
    let mut verticle_column_num:u32 = 0;
    for verticle_row in verticle_rows{
        let verticle_plot_len: u32 = verticle_row.len() as u32;
        visible_trees
            .append(&mut get_visible_trees(
                &verticle_row,
                verticle_column_num, 
                verticle_plot_len, 
                false,
                true
            ));
            let reverse_verticle_row: &str = &verticle_row.chars().rev().collect::<String>();
            visible_trees
            .append(&mut get_visible_trees(
                reverse_verticle_row,
                verticle_column_num, 
                verticle_plot_len, 
                true,
                true
            ));
            verticle_column_num += 1;
    }

    //Get all the unique coordinates
    if is_debug{
        println!("Debug before hash {}", visible_trees.len());
        println!("Debug here: {:?}", visible_trees);
    }
    let visible_trees_hash: HashSet<String> = visible_trees.into_iter().collect();
    let visible_tree_count:u32 = visible_trees_hash.len().try_into().unwrap();
    if is_debug{
        println!("Debug after hash {}", visible_tree_count);
        println!("Debug hash: {:?}", visible_trees_hash);
    }

    return visible_tree_count;
}