#[path="../utils/mod.rs"]
mod utils;
use utils::elf::sum_calories;

#[test]
fn test_sum_calories_should_return_6000_first_example(){
    const EXPECTED_SUM: u32 = 6000;
    let mut elf_calories = Vec::new();
    elf_calories.push(1000);
    elf_calories.push(2000);
    elf_calories.push(3000);

    let elf_sum = sum_calories(elf_calories);

    assert_eq!(elf_sum, EXPECTED_SUM);
}
#[test]
fn test_sum_calories_should_return_sum_one_snack(){
    const EXPECTED_SUM: u32 = 4000;
    let mut elf_calories = Vec::new();
    elf_calories.push(4000);

    let elf_sum = sum_calories(elf_calories);

    assert_eq!(elf_sum, EXPECTED_SUM);
}