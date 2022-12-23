pub fn get_visible_trees(tree_row: &str, row_coord: u32, plot_len: u32)-> Vec<u32>{
    if row_coord == 0 || row_coord == plot_len -1{
        return tree_row
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
    }
    else{
        let mut highest_tree_so_far: i32 = -1;
        let mut visible_trees = Vec::new();
        for size in tree_row.chars().map(|c| c.to_digit(10).unwrap()){
            if size as i32 > highest_tree_so_far {
                highest_tree_so_far = size as i32;
                visible_trees.push(size);
            }
        }
        return visible_trees;
    }
}