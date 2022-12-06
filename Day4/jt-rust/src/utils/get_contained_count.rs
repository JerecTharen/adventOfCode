use super::{pair_partner::build_pair_partner, check_sections::{is_any_overlap, is_inside}};



pub fn get_contained_count(file_stuff: String, contained_counter: &mut u32, is_any_overlap_counted: bool) {
    file_stuff.split('\n').for_each(|line|{
        let mut pairs = line.split(',').map(|pair_string|{
            let mut pair_split = pair_string.split('-');
            let first = pair_split.next().unwrap().to_string().parse().unwrap();
            let second = pair_split.next().unwrap().to_string().parse().unwrap();
            build_pair_partner(first, second)
        });
        let first = pairs.next().unwrap();
        let second = pairs.next().unwrap();
        if is_inside(first, second) || is_inside(second, first){
            println!("This pair contains the other ({}, {}) and ({}, {})", first.section_start, first.section_end, second.section_start, second.section_end);
            *contained_counter = *contained_counter + 1_u32
        }
        else if is_any_overlap_counted && (is_any_overlap(first, second) || is_any_overlap(second, first)){
            println!("This pair has some overlap: ({}, {}) and ({}, {})", first.section_start, first.section_end, second.section_start, second.section_end);
            *contained_counter = *contained_counter + 1_u32
        }
    });
}