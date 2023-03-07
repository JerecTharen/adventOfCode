mod utils;
mod tests;
use std::env;
use std::fs;
use std::path::Path;

use utils::elf;
use utils::input_parser;
use utils::distributer;

fn main() {
    println!("Hello There!");
    let path = Path::new("./ignore_data/input.txt");
    // let path = Path::new("./ignore_data/example.txt");
    if !path.is_file(){
        println!("Error: cannot find file!");
        println!("Current directory {:?}", env::current_dir().unwrap());
        return;
    }
    println!("Debug path: {0}", path.is_file());
    let input: String = fs::read_to_string(path).unwrap();
    let elf_packs = input_parser::parse_input(input);
    let mut fattest_elf = 0;
    let mut elves = Vec::new();
    for elf in elf_packs {
        let sum = elf::sum_calories(elf);
        if sum > fattest_elf{
            fattest_elf = sum;
        }
        elves.push(sum);
    }
    let top_pack_sum:u32 = distributer::get_top_packs(elves).iter().sum();

    println!("Puzzle answer part 1 is: {0}", fattest_elf);
    println!("Puzzle answer part 2 is: {0}", top_pack_sum);
}
