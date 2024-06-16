use crate::types::*;

pub const MAJOR_CITIES: [&str; 23] = [
    "MES", "CAT", "SIR", "RAG", "AGR", "CTN", "ENN", "PMO", "TRA", "RCA", "VVA", "CNZ", "CRO",
    "COS", "POT", "MAT", "NAP", "BEN", "SRN", "SAP", "AVE", "FOG", "BRL"
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
