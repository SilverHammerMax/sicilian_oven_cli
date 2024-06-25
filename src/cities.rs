use crate::types::city::Region;
use crate::types::city::{City, RoadTypes};
use std::collections::BTreeSet;

pub struct CityConnection {
    cities: (String, String),
    distance: i32,
    road: RoadTypes,
}

impl CityConnection {
    fn new(cities: (&str, &str), distance: i32, road: RoadTypes) -> CityConnection {
        CityConnection {
            cities: (cities.0.to_string(), cities.1.to_string()),
            distance,
            road,
        }
    }
}

pub struct CityGraph {
    cities: BTreeSet<City>,
    connections: Vec<CityConnection>,
}

impl CityGraph {
    pub fn get_neighbors(&self, city_name: &str) -> Vec<(String, i32, RoadTypes)> {
        let neighbors = self
            .connections
            .iter()
            .filter(|connection| {
                connection.cities.0 == city_name || connection.cities.1 == city_name
            })
            .map(|connection| {
                if connection.cities.0 == city_name {
                    (
                        connection.cities.1.clone(),
                        connection.distance,
                        connection.road,
                    )
                } else {
                    (
                        connection.cities.0.clone(),
                        connection.distance,
                        connection.road,
                    )
                }
            })
            .collect();
        neighbors
    }

    fn add_city(&mut self, city: City) {
        self.cities.insert(city);
    }

    fn add_connection(&mut self, connection: CityConnection) {
        self.connections.push(connection);
    }

    pub fn get(&self, city_name: &String) -> Option<&City> {
        self.cities.get(city_name)
    }

    pub fn cities(&self) -> &BTreeSet<City> {
        &self.cities
    }
}

pub fn create_cities() -> CityGraph {
    let mut graph = CityGraph {
        cities: BTreeSet::new(),
        connections: vec![],
    };
    graph.add_city(City::new("Ragusa", Region::Sicily, true));
    graph.add_city(City::new("Comiso", Region::Sicily, false));
    graph.add_city(City::new("Vittoria", Region::Sicily, false));
    graph.add_city(City::new("Marina di Ragusa", Region::Sicily, false));
    graph.add_city(City::new("Pozzallo", Region::Sicily, false));
    graph.add_city(City::new("Modica", Region::Sicily, false));
    graph.add_city(City::new("Capo Passero", Region::Sicily, false));

    graph.add_connection(CityConnection::new(
        ("Ragusa", "Comiso"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Comiso", "Vittoria"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Marina di Ragusa"),
        33,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Marina di Ragusa", "Pozzallo"),
        31,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Modica"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Pozzallo", "Modica"),
        30,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Capo Passero", "Pozzallo"),
        40,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Capo Passero", "Modica"),
        40,
        RoadTypes::Asphalt,
    ));

    graph
}
