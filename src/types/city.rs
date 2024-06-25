use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub enum RoadTypes {
    Highway,
    Asphalt,
    Cobblestone,
    Unpaved,
    Ferry,
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd)]
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

#[derive(Ord, Eq, PartialOrd, Clone)]
pub struct City {
    city_name: String,
    region: Region,
    refuel: bool,
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.city_name == other.city_name
    }
}

impl Hash for City {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.city_name.hash(state);
    }
}

impl Borrow<String> for City {
    fn borrow(&self) -> &String {
        &self.city_name
    }
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.city_name, self.region)
    }
}

impl City {
    pub const fn new(
        city_name: String,
        region: Region,
        refuel: bool,
    ) -> City {
        City {
            city_name,
            region,
            refuel,
        }
    }

    pub fn get_name(&self) -> &str {
        self.city_name.as_str()
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }

    pub fn is_major(&self) -> bool {
        self.refuel
    }
}

pub fn major_cities(region: Option<&Region>) -> Vec<String> {
    crate::cities::create_cities().cities
        .iter()
        .filter(|city| {
            city.is_major() && (region.is_none() || Some(city.get_region()) == region)
        })
        .map(|city| city.get_name().to_string())
        .collect()
}
