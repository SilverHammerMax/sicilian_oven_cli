pub enum RoadTypes {
    Highway,
    Asphalt,
    Cobblestone,
    Gravel,
    Ferry
}

impl RoadTypes {
    pub fn time_multiplier(&self) -> f64 {
        match self {
            RoadTypes::Highway => 0.5,
            RoadTypes::Asphalt => 1.0,
            RoadTypes::Cobblestone => 4.0 / 3.0,
            RoadTypes::Gravel => 2.0,
            RoadTypes::Ferry => 0.0
        }
    }
}

pub struct City {
    city_name: &'static str,
    connected_cities: &'static [(&'static str, i32, RoadTypes)],
    refuel: bool,
}

impl City {
    pub const fn new(
        city_name: &'static str,
        connected_cities: &'static [(&'static str, i32, RoadTypes)],
        refuel: bool,
    ) -> City {
        City {
            city_name,
            connected_cities,
            refuel,
        }
    }

    pub fn get_name(&self) -> &str {
        self.city_name
    }

    pub fn get_cities(&self) -> &'static [(&'static str, i32, RoadTypes)] {
        self.connected_cities
    }

    pub fn can_refuel(&self) -> bool {
        self.refuel
    }
}
