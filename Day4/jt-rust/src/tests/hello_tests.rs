#[path ="../utils/mod.rs"]
mod utils;
use utils::hello::hello::respond_hello as respond_hello;

#[test]
fn test_response_hello_should_return_phrase(){
    const EXPECTED_RESPONSE: &str = "General Kenobi!";

    let actual_response = respond_hello();

    assert_eq!(actual_response, EXPECTED_RESPONSE);
}