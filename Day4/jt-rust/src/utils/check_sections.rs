// #[path ="./pair_partner.rs"]
// pub mod pair_partner;
// pub use pair_partner::PairPartner as PairPartner;

use super::PairPartner;

pub fn is_inside(first_partner: PairPartner, second_partner: PairPartner)->bool{
    first_partner.section_start < second_partner.section_start && first_partner.section_end > second_partner.section_end
}