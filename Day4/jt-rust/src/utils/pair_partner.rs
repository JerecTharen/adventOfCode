#[derive(Clone, Copy)]
pub struct PairPartner{
    pub section_start: u32,
    pub section_end: u32
}

pub fn build_pair_partner(section_start: u32, section_end: u32)-> PairPartner{
    PairPartner{
        section_start: section_start,
        section_end: section_end
    }
}