use crate::types::medal::Medal;
use crate::types::*;
use strum;

#[derive(strum::EnumIter, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Challenge {
    RagusanRide,
    BigCarBigCities,
    ARideAroundMountEtna,
    TheGodfather,
    FreePlay,
}

impl Challenge {
    pub fn get_name(&self) -> &str {
        match self {
            Challenge::RagusanRide => "Ragusan Ride",
            Challenge::BigCarBigCities => "Big Car, Big Cities",
            Challenge::ARideAroundMountEtna => "A Ride Around Mount Etna",
            Challenge::TheGodfather => "The Godfather",
            Challenge::FreePlay => "Free Play",
        }
    }
    pub fn get_car(&self) -> Option<car::Car> {
        match self {
            Challenge::RagusanRide => Some(car::Car::new(car::CarType::Lancia)),
            Challenge::BigCarBigCities => Some(car::Car::new(car::CarType::Ferrari)),
            Challenge::ARideAroundMountEtna => Some(car::Car::new(car::CarType::Maserati)),
            Challenge::TheGodfather => Some(car::Car::new(car::CarType::ModifiedLancia)),
            Challenge::FreePlay => None,
        }
    }

    pub fn get_cities(&self) -> Vec<&str> {
        match self {
            Challenge::RagusanRide => vec![
                "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO",
                "AUG", "LEN",
            ],
            Challenge::BigCarBigCities => vec![
                "RAG", "SIR", "CAT", "ENN", "CAL", "PMO", "TRA", "MES", "AGR",
            ],
            Challenge::ARideAroundMountEtna => vec![
                "CAT", "GER", "PAT", "ADR", "RAN", "CRL", "PTI", "BAR", "MIL", "MES", "RIP", "TAM",
                "ACI", "LEN", "NIC", "ENN",
            ],
            Challenge::TheGodfather => vec![
                "COR", "SEL", "MAR", "CST", "PAR", "MEN", "SCI", "POR", "AGR", "RIB", "CAN", "LIC",
            ],
            Challenge::FreePlay => vec![],
        }
    }

    pub fn get_starting_city(&self) -> Option<&str> {
        match self {
            Challenge::RagusanRide => Some("RAG"),
            Challenge::BigCarBigCities => None,
            Challenge::ARideAroundMountEtna => Some("CAT"),
            Challenge::TheGodfather => Some("COR"),
            Challenge::FreePlay => None,
        }
    }

    pub fn get_ending_city(&self) -> Option<&str> {
        match self {
            Challenge::RagusanRide => Some("RAG"),
            Challenge::BigCarBigCities => None,
            Challenge::ARideAroundMountEtna => Some("CAT"),
            Challenge::TheGodfather => Some("COR"),
            Challenge::FreePlay => None,
        }
    }

    pub fn get_medal(&self, time: f64) -> Medal {
        match self {
            Challenge::RagusanRide => match time {
                0.0..=205.0 => Medal::Author,
                0.0..=220.0 => Medal::Gold,
                0.0..=270.0 => Medal::Silver,
                0.0..=330.0 => Medal::Bronze,
                _ => Medal::None,
            },
            Challenge::BigCarBigCities => match time {
                0.0..=310.0 => Medal::Author,
                0.0..=325.0 => Medal::Gold,
                0.0..=375.0 => Medal::Silver,
                0.0..=475.0 => Medal::Bronze,
                _ => Medal::None,
            },
            Challenge::ARideAroundMountEtna => match time {
                0.0..=290.0 => Medal::Author,
                0.0..=310.0 => Medal::Gold,
                0.0..=335.0=> Medal::Silver,
                0.0..=395.0 => Medal::Bronze,
                _ => Medal::None,
            },
            Challenge::TheGodfather => match time {
                0.0..=305.0 => Medal::Author,
                0.0..=325.0 => Medal::Gold,
                0.0..=370.0 => Medal::Silver,
                0.0..=450.0 => Medal::Bronze,
                _ => Medal::None,
            },
            Challenge::FreePlay => Medal::None,
        }
    }
}
