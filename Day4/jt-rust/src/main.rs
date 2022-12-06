use std::fs;
mod utils;
mod tests;
use utils::hello::hello::respond_hello as respond_hello;
use utils::{get_contained_count};

fn main() {
    println!("Hello There!");
    respond_hello();
    let file_stuff = fs::read_to_string("./ignore_data/input.txt").expect("Cannot read input file");
    let file_stuff_2_electric_bugaloo = String::clone(&file_stuff);
    println!("{} characters long", file_stuff.len());

    let mut cointained_counter: u32 = 0;

    get_contained_count(file_stuff, &mut cointained_counter, false);

    println!("Counter, at the end, is at: {}", cointained_counter);


    let mut cointained_counter_any_overlap: u32 = 0;

    get_contained_count(file_stuff_2_electric_bugaloo, &mut cointained_counter_any_overlap, true);

    println!("Counter, at the end for all overlap, is at: {}", cointained_counter_any_overlap);
}

