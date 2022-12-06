use super::{pair_partner::build_pair_partner, check_sections};



pub fn get_contained_count(file_stuff: String, contained_counter: &mut u32) {
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
            println!("This pair contains the other ({}, {}) and ({}, {})", first.section_start, first.section_end, second.section_start, second.section_end);
            *contained_counter = *contained_counter + 1_u32
        }
    });
}