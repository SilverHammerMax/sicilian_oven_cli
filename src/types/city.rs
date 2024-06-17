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

impl Region {
    pub fn get_name(&self) -> &str {
        match self {
            Region::Sicily => "Sicily",
            Region::Calabria => "Calabria",
            Region::Basilicata => "Basilicata",
            Region::Apulia => "Apulia",
            Region::Campania => "Campania",
            Region::Molise => "Molise",
            Region::Abruzzo => "Abruzzo",
            Region::Lazio => "Lazio",
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

pub fn major_cities() -> Vec<&'static str> {
    crate::cities::CITIES.iter().filter(|(code, city)| city.is_major()).map(|(code, city)| *code).collect()
}
