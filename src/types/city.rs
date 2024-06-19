use std::fmt::{Display, Formatter};

pub enum RoadTypes {
    Highway,
    Asphalt,
    Cobblestone,
    Unpaved,
    Ferry,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Region {
    Sicily,
    Calabria,
    Basilicata,
    Apulia,
    Campania,
    Molise,
    Abruzzo,
    Lazio,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::Sicily => write!(f, "Sicily"),
            Region::Calabria => write!(f, "Calabria"),
            Region::Basilicata => write!(f, "Basilicata"),
            Region::Apulia => write!(f, "Apulia"),
            Region::Campania => write!(f, "Campania"),
            Region::Molise => write!(f, "Molise"),
            Region::Abruzzo => write!(f, "Abruzzo"),
            Region::Lazio => write!(f, "Lazio"),
        }
    }
}
pub struct City {
    city_name: &'static str,
    region: Region,
    connected_cities: &'static [(&'static str, i32, RoadTypes)],
    refuel: bool,
}

impl City {
    pub const fn new(
        city_name: &'static str,
        region: Region,
        connected_cities: &'static [(&'static str, i32, RoadTypes)],
        refuel: bool,
    ) -> City {
        City {
            city_name,
            region,
            connected_cities,
            refuel,
        }
    }

    pub fn get_name(&self) -> &str {
        self.city_name
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }

    pub fn get_cities(&self) -> &'static [(&'static str, i32, RoadTypes)] {
        self.connected_cities
    }

    pub fn is_major(&self) -> bool {
        self.refuel
    }
}

pub fn major_cities(region: Option<&Region>) -> Vec<&'static str> {
    crate::cities::CITIES.iter().filter(|(code, city)| city.is_major() && (region.is_none() || Some(city.get_region()) == region)).map(|(code, city)| *code).collect()
}
