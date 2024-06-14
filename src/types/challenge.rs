use crate::types::*;

#[derive(PartialEq, Eq, Clone)]
pub enum Location {
    City(&'static str),
    Region(city::Region),
    Any
}

#[derive(Clone)]
pub struct Challenge {
    name: &'static str,
    car: Option<car::Car>,
    cities: &'static [&'static str],
    start_city: Location,
    end_city: Location,
    medal_cutoffs: [i32; 4],
    medal: medal::Medal,
}

pub const CHALLENGES: [Challenge; 1] = [
    Challenge::new("Ragusan Ride", Some(car::Car::new(car::CarType::Lancia)), &[
        "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO",
        "AUG", "LEN",
    ], Location::City("RAG"), Location::City("RAG"), [205, 220, 270, 330])
];

impl Challenge {
    pub const fn new(name: &'static str, car: Option<car::Car>, cities: &'static [&'static str], start_city: Location, end_city: Location, medal_cutoffs: [i32; 4]) -> Challenge {
        Challenge {
            name,
            car,
            cities,
            start_city,
            end_city,
            medal_cutoffs,
            medal: medal::Medal::None,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_car(&self) -> Option<car::Car> {
        self.car.clone()
    }

    pub fn get_cities(&self) -> &[&str] {
        self.cities
    }

    pub fn get_start_city(&self) -> &Location {
        &self.start_city
    }

    pub fn get_end_city(&self) -> &Location {
        &self.end_city
    }

    pub fn get_medal_cutoff(&self, index: usize) -> i32 {
        self.medal_cutoffs[index]
    }

    pub fn get_medal(&self) -> &medal::Medal {
        &self.medal
    }

    pub fn set_medal(&mut self, medal: medal::Medal) {
        self.medal = medal;
    }
}
