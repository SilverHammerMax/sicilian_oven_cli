use lazy_static::lazy_static;
use std::collections::HashMap;

pub const MAJOR_CITIES: [&str; 9] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA",
];

lazy_static! {
    pub static ref MEDAL_CUTOFFS: HashMap<&'static crate::types::challenge::Challenge, [i32; 4]> =
        HashMap::from([
            (
                &crate::types::challenge::Challenge::RagusanRide,
                [205, 220, 270, 330]
            ),
            (
                &crate::types::challenge::Challenge::BigCarBigCities,
                [310, 325, 375, 475]
            ),
            (
                &crate::types::challenge::Challenge::ARideAroundMountEtna,
                [290, 310, 335, 395]
            ),
            (
                &crate::types::challenge::Challenge::TheGodfather,
                [305, 325, 370, 450]
            )
        ]);
}
