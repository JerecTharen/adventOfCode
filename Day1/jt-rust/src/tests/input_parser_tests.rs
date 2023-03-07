#[path="../utils/mod.rs"]
mod utils;
use utils::input_parser::parse_input;


#[test]
fn test_parse_input_should_parse_example(){
    #[allow(non_snake_case)]
    let EXPECTED_ELF_VECTORS:Vec<Vec<u32>> = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]];
    let input: String = "1000
2000
3000
    
4000
    
5000
6000
    
7000
8000
9000
    
10000".to_string();

    let actual_elf_vectors: Vec<Vec<u32>> = parse_input(input);

    assert_eq!(actual_elf_vectors, EXPECTED_ELF_VECTORS);
}