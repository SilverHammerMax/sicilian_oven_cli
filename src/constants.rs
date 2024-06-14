use crate::types::*;

pub const MAJOR_CITIES: [&str; 15] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA", "RCA", "VVA", "CNZ", "CRO",
    "COS", "NAP",
];

pub const CHALLENGES: [challenge::Challenge; 2] = [
    challenge::Challenge::new(
    "Ragusan Ride",
    Some(car::Car::new(car::CarType::Lancia)),
    &[
        "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO", "AUG",
        "LEN",
    ],
    challenge::Location::City("RAG"),
    challenge::Location::City("RAG"),
    [205, 220, 270, 330],
),
    challenge::Challenge::new(
    "Big Car, Big Cities",
    Some(car::Car::new(car::CarType::Ferrari)),
    &["RAG", "SIR", "CAT", "ENN", "CTN", "PMO", "TRA", "MES", "AGR"],
    challenge::Location::Region(city::Region::Sicily),
    challenge::Location::Region(city::Region::Sicily),
    [310, 325, 375, 475],
)];
