mod utils;
mod tests;
use std::fs;
use utils::hello::hello::respond_hello as respond_hello;

use crate::utils::tree_counter::count_visible_trees;

fn main() {
    println!("Hello There!");
    respond_hello();

    //Read each line
    let file_stuff = fs
        ::read_to_string("./ignore_data/input.txt")
        .expect("Cannot read input file");
    
    println!("{} Characters long", file_stuff.len());

    let part_one_visible_trees = count_visible_trees(file_stuff, false);
    //1784 is too low :(
    println!("Part 1 answer is: {}", part_one_visible_trees);

    
}
