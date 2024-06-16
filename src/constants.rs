use crate::types::*;

pub const MAJOR_CITIES: [&str; 23] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA", "RCA", "VVA", "CNZ", "CRO",
    "COS", "POT", "MAT", "NAP", "BEN", "SRN", "SAP", "AVE", "FOG", "BRL"
];

pub const CHALLENGES: [challenge::Challenge; 7] = [
    challenge::Challenge::new(
        "Ragusan Ride",
        Some(CARS[3]),
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
        Some(CARS[0]),
        &[
            "RAG", "SIR", "CAT", "ENN", "CTN", "PMO", "TRA", "MES", "AGR",
        ],
        challenge::Location::Region(city::Region::Sicily),
        challenge::Location::Region(city::Region::Sicily),
        [310, 325, 375, 475],
    ),
    challenge::Challenge::new(
        "A Ride Around Mt. Etna",
        Some(CARS[2]),
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
        Some(CARS[3]),
        &[
            "COR", "SEL", "MAR", "CST", "PAR", "MEN", "SCI", "POR", "AGR", "RIB", "CAN", "LIC",
        ],
        challenge::Location::City("COR"),
        challenge::Location::City("COR"),
        [305, 325, 370, 395],
    ),
    challenge::Challenge::new(
        "Harbormaster",
        None,
        &[
            "ISC", "STR", "LIP", "MAL", "FAV", "PAN", "TRC"
        ],
        challenge::Location::Any,
        challenge::Location::Any,
        [0, 0, 0, 0],
    ),
    challenge::Challenge::new(
        "A Calabrian Rally",
        Some(CARS[1]),
        &[
            "ACR", "COT", "ORI", "DIN", "DEL"
        ],
        challenge::Location::City("CNZ"),
        challenge::Location::Any,
        [0, 0, 0, 0],
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

pub const CARS: [car_parts::car::Car; 4] = [
    car_parts::car::Car::new(
        "Il Comandante",
        car_parts::tire::Tire::Four,
        car_parts::engine::Engine::One,
        car_parts::gearbox::Gearbox::Three,
        car_parts::chassis::Chassis::Three,
    ),
    car_parts::car::Car::new(
        "Il Grande",
        car_parts::tire::Tire::Three,
        car_parts::engine::Engine::Five,
        car_parts::gearbox::Gearbox::Four,
        car_parts::chassis::Chassis::Five,
    ),
    car_parts::car::Car::new(
        "Il Capo",
        car_parts::tire::Tire::Two,
        car_parts::engine::Engine::Two,
        car_parts::gearbox::Gearbox::Two,
        car_parts::chassis::Chassis::One,
    ),
    car_parts::car::Car::new(
        "Il Generalissimo",
        car_parts::tire::Tire::Four,
        car_parts::engine::Engine::Three,
        car_parts::gearbox::Gearbox::One,
        car_parts::chassis::Chassis::Two,
    ),
];
