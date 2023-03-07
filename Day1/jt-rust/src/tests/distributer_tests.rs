#[path="../utils/mod.rs"]
mod utils;

#[test]
fn test_get_top_packs_does_example(){
    const EXPECTED_TOP_PACKS: [u32; 3] = [24000, 11000, 10000];
    let elves = vec![6000, 4000, 11000, 24000, 10000];

    let actual_top_packs = utils::distributer::get_top_packs(elves);

    assert_eq!(actual_top_packs, EXPECTED_TOP_PACKS);
}