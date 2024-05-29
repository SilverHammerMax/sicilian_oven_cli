pub enum RoadTypes {
    Highway,
    Asphalt,
    Cobblestone,
    Gravel
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
