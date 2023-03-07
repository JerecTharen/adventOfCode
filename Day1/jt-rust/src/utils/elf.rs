
pub fn sum_calories(calories: Vec<u32>)->u32{
    let mut sum: u32 = 0;
    for snack in calories {
        sum += snack;
    }
    sum
}