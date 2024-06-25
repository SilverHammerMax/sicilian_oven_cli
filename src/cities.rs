use crate::types::city::Region;
use crate::types::city::{City, RoadTypes};
use std::collections::BTreeSet;

pub struct CityGraph {
    pub cities: BTreeSet<City>,
    pub connections: Vec<(String, String, i32, RoadTypes)>,
}

impl CityGraph {
    pub fn get_neighbors(&self, city_name: &str) -> Vec<(String, i32, RoadTypes)> {
        let neighbors = self.connections.iter().filter(|connection| connection.0 == city_name || connection.1 == city_name).map(|connection| {
            if connection.0 == city_name {
                (connection.1.clone(), connection.2, connection.3)
            } else {
                (connection.0.clone(), connection.2, connection.3)
            }
        }).collect();
        neighbors
    }
}

pub fn create_cities() -> CityGraph {
    let mut graph = CityGraph {
        cities: BTreeSet::new(),
        connections: vec![],
    };
    graph.cities.insert(City::new("Ragusa".to_string(), Region::Sicily, true));
    graph.cities.insert(City::new("Comiso".to_string(), Region::Sicily, false));
    graph.cities.insert(City::new("Vittoria".to_string(), Region::Sicily, false));

    graph.connections.push(("Ragusa".to_string(), "Comiso".to_string(), 8, RoadTypes::Cobblestone));
    graph.connections.push(("Comiso".to_string(), "Vittoria".to_string(), 8, RoadTypes::Cobblestone));

    graph
}