#[path ="../utils/mod.rs"]
mod utils;
use utils::check_sections::is_inside as is_inside;
use utils::pair_partner::build_pair_partner as build_pair_partner;

#[test]
fn test_is_inside_should_be_able_to_return_true(){
    const EXPECTED_RESPONSE: bool = true;
    let stub_first_start = 1;
    let stub_first_end = 5;
    let stub_first_partner = build_pair_partner(stub_first_start, stub_first_end);
    
    let stub_second_start = stub_first_start + 1;
    let stub_second_end = stub_first_end - 1;
    let stub_second_partner = build_pair_partner(stub_second_start, stub_second_end);

    let actual_response = is_inside(stub_first_partner, stub_second_partner);

    assert_eq!(actual_response, EXPECTED_RESPONSE);
}
#[test]
fn test_is_inside_should_be_able_to_return_false(){
    const EXPECTED_RESPONSE: bool = false;
    let stub_first_start = 1;
    let stub_first_end = 5;
    let stub_first_partner = build_pair_partner(stub_first_start, stub_first_end);
    
    let stub_second_start = stub_first_start - 1;
    let stub_second_end = stub_first_end + 1;
    let stub_second_partner = build_pair_partner(stub_second_start, stub_second_end);

    let actual_response = is_inside(stub_first_partner, stub_second_partner);

    assert_eq!(actual_response, EXPECTED_RESPONSE);
}
#[test]
fn test_is_inside_should_be_able_to_return_true_for_equivillent_value(){
    const EXPECTED_RESPONSE: bool = true;
    let stub_first_start = 1;
    let stub_first_end = 5;
    let stub_first_partner = build_pair_partner(stub_first_start, stub_first_end);
    
    let stub_second_start = stub_first_start;
    let stub_second_end = stub_first_end;
    let stub_second_partner = build_pair_partner(stub_second_start, stub_second_end);

    let actual_response = is_inside(stub_first_partner, stub_second_partner);

    assert_eq!(actual_response, EXPECTED_RESPONSE);
}