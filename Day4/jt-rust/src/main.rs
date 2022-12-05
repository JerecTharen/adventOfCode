use std::fs;
mod utils;
mod tests;
use utils::hello::hello::respond_hello as respond_hello;
use utils::pair_partner::build_pair_partner as build_pair_partner;

use crate::utils::check_sections;

fn main() {
    println!("Hello There!");
    respond_hello();
    let file_stuff = fs::read_to_string("./ignore_data/example.txt").expect("Cannot read example file");
    println!("{}", file_stuff);

    file_stuff.split('\n').for_each(|line|{
        let mut pairs = line.split(',').map(|pair_string|{
            let mut pair_split = pair_string.split('-');
            let first = pair_split.next().unwrap().to_string().parse().unwrap();
            let second = pair_split.next().unwrap().to_string().parse().unwrap();
            build_pair_partner(first, second)
        });
        let first = pairs.next().unwrap();
        let second = pairs.next().unwrap();
        if check_sections::is_inside(first, second) || check_sections::is_inside(second, first){
            println!("This pair worked");
        }
    })
}
