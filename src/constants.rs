use std::string::ToString;
use crate::types::*;

pub const MAJOR_CITIES: [&str; 15] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA", "RCA", "VVA", "CNZ", "CRO",
    "COS", "NAP",
];

pub const CHALLENGES: [challenge::Challenge; 5] = [
    challenge::Challenge::new(
        "Ragusan Ride",
        Some(car::car::Car::new(car::CarType::Lancia)),
        &[
            "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO",
            "AUG", "LEN",
        ],
        challenge::Location::City("RAG"),
        challenge::Location::City("RAG"),
        [205, 220, 270, 330],
    ),
    challenge::Challenge::new(
        "Big Car, Big Cities",
        Some(car::car::Car::new(car::CarType::Ferrari)),
        &[
            "RAG", "SIR", "CAT", "ENN", "CTN", "PMO", "TRA", "MES", "AGR",
        ],
        challenge::Location::Region(city::Region::Sicily),
        challenge::Location::Region(city::Region::Sicily),
        [310, 325, 375, 475],
    ),
    challenge::Challenge::new(
        "A Ride Around Mt. Etna",
        Some(car::car::Car::new(car::CarType::Maserati)),
        &[
            "CAT", "GER", "PAT", "ADR", "RAN", "CRL", "PTI", "BAR", "MIL", "MES", "RIP", "TAM",
            "ACI", "LEN", "NIC", "ENN",
        ],
        challenge::Location::City("CAT"),
        challenge::Location::City("CAT"),
        [290, 310, 335, 395],
    ),
    challenge::Challenge::new(
        "The Godfather",
        Some(car::car::Car::new(car::CarType::ModifiedLancia)),
        &[
            "COR", "SEL", "MAR", "CST", "PAR", "MEN", "SCI", "POR", "AGR", "RIB", "CAN", "LIC",
        ],
        challenge::Location::City("COR"),
        challenge::Location::City("COR"),
        [305, 325, 370, 395],
    ),
    challenge::Challenge::new(
        "Free Play",
        None,
        &[],
        challenge::Location::Any,
        challenge::Location::Any,
        [0, 0, 0, 0],
    ),
];

pub const CARS: [car::car::Car; 1] = [
    car::car::Car::new("Il Comandante".to_string(), car::tire::Tire::TireFour, car::engine::Engine::EngineOne, car::gearbox::Gearbox::GearboxThree, car::chassis::Chassis::ChassisThree),
];