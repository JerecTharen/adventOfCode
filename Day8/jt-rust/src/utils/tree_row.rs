
pub fn get_visible_trees(
    tree_row: &str, 
    row_coord: u32, 
    plot_len: u32, 
    is_reverse: bool, 
    is_verticle: bool
)-> Vec<String>{
    let mut column_num = -1_i32;
    let mut column_incrementer: i32 = 1;
    let mut row_num:i32 = row_coord as i32;
    if is_reverse && !is_verticle{//Allow counting backwards
        column_num = tree_row.chars().count() as i32;
        column_incrementer = -1;
    }
    else if is_verticle && !is_reverse{
        column_num = row_coord as i32;
        row_num = -1;
    }
    else if is_verticle && is_verticle{
        column_num = row_coord as i32;
        row_num = tree_row.chars().count() as i32;
        column_incrementer = -1;
    }

    if row_coord == 0 || row_coord == plot_len -1{
        tree_row
            .chars()
            .map(|c| {
                if !is_verticle{
                    column_num += column_incrementer;

                }
                else{
                    row_num += column_incrementer;
                }
                let size:u32 = c.to_digit(10).unwrap();
                create_tree_coord(row_num as u32, column_num, size)
            })
            .collect()
    }
    else{
        let mut highest_tree_so_far: i32 = -1;
        let mut visible_trees = Vec::new();
        for size in tree_row.chars().map(|c| c.to_digit(10).unwrap()){
            if !is_verticle{
                column_num += column_incrementer;
            }
            else{
                row_num += column_incrementer;
            }
            if size as i32 > highest_tree_so_far {
                highest_tree_so_far = size as i32;
                visible_trees.push(create_tree_coord(row_num as u32, column_num, size));
            }
        }
        return visible_trees;
    }
}

pub fn create_tree_coord(row_coord: u32, column_num: i32, size: u32)->String{
    return format!("{}{}{}", row_coord, column_num, size);
}

pub fn get_verticle_rows(tree_rows: Vec<&str>)-> Vec<String>{
    let row_len = tree_rows[0].chars().count();
    let mut vertical_rows: Vec<String> = Vec::new();
    for index in 0..row_len {
        let mut verticle_row: Vec<char> = Vec::new();
        tree_rows
            .iter()
            .for_each(|tree_row| {
                let current_size = tree_row.chars().nth(index);
                if current_size.is_some(){
                    verticle_row.push(tree_row.chars().nth(index).unwrap());
                } else{
                    panic!("Cannot get current size for row: {:?}", verticle_row);
                }
            });
        vertical_rows.push(String::from_iter(verticle_row));
    }

    return vertical_rows;
}