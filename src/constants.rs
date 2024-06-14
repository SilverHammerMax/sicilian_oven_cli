use crate::types::car;
use crate::types::challenge::{Challenge, Location};

pub const MAJOR_CITIES: [&str; 15] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA", "RCA", "VVA", "CNZ", "CRO", "COS", "NAP"
];

pub const CHALLENGES: [Challenge; 1] = [Challenge::new(
    "Ragusan Ride",
    Some(car::Car::new(car::CarType::Lancia)),
    &[
        "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO", "AUG",
        "LEN",
    ],
    Location::City("RAG"),
    Location::City("RAG"),
    [205, 220, 270, 330],
)];
