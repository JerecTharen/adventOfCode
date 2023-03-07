
pub fn get_top_packs(mut elves: Vec<u32>)-> [u32; 3]{
    elves.sort();
    elves.reverse();
    let mut top_packs = Vec::new();
    for elf in elves {
        if top_packs.len() != 3{
            top_packs.push(elf);
        }
    }

    top_packs.try_into().unwrap()
}