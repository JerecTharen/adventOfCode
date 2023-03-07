
pub fn parse_input(all_elfs: String) -> Vec<Vec<u32>>{
    let lines:Vec<&str> = all_elfs.split("\n").collect();
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for (index, snack) in lines.iter().enumerate() {
        if snack.trim() != "" && index != lines.len()-1{
            elf.push(snack.parse::<u32>().unwrap());
        }
        else if index == lines.len()-1{
            elf.push(snack.parse::<u32>().unwrap());
            elves.push(elf.clone());
        }
        else{
            elves.push(elf.clone());
            elf = Vec::new();
        }
    }

    elves
}