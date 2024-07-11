use crate::types::city::Region;
use crate::types::city::{City, RoadTypes};
use std::collections::BTreeSet;
use bevy::prelude::Resource;

pub struct CityConnection {
    cities: (String, String),
    distance: i32,
    road: RoadTypes,
}

impl CityConnection {
    fn new<T: Into<String>>(cities: (T, T), distance: i32, road: RoadTypes) -> CityConnection {
        CityConnection {
            cities: (cities.0.into(), cities.1.into()),
            distance,
            road,
        }
    }
}

#[derive(Resource)]
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

    pub fn get<T: Into<String>>(&self, city_name: T) -> Option<&City> {
        self.cities.get(&city_name.into())
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
    // Adding cities to the graph
    graph.add_city(City::new("Ragusa", Region::Sicily, true));
    graph.add_city(City::new("Comiso", Region::Sicily, false));
    graph.add_city(City::new("Vittoria", Region::Sicily, false));
    graph.add_city(City::new("Marina di Ragusa", Region::Sicily, false));
    graph.add_city(City::new("Pozzallo", Region::Sicily, false));
    graph.add_city(City::new("Modica", Region::Sicily, false));
    graph.add_city(City::new("Capo Passero", Region::Sicily, false));
    graph.add_city(City::new("Noto Marioa", Region::Sicily, false));
    graph.add_city(City::new("Siracusa", Region::Sicily, true));
    graph.add_city(City::new("Augusta", Region::Sicily, false));
    graph.add_city(City::new("Lentini", Region::Sicily, false));
    graph.add_city(City::new("Floridia", Region::Sicily, false));
    graph.add_city(City::new("Palazzolo Acreide", Region::Sicily, false));
    graph.add_city(City::new("Giarratana", Region::Sicily, false));
    graph.add_city(City::new("Gela", Region::Sicily, false));
    graph.add_city(City::new("Caltagirone", Region::Sicily, false));
    graph.add_city(City::new("Gerbini", Region::Sicily, false));
    graph.add_city(City::new("Paterno", Region::Sicily, false));
    graph.add_city(City::new("Catania", Region::Sicily, true));
    graph.add_city(City::new("Acireale", Region::Sicily, false));
    graph.add_city(City::new("Riposte", Region::Sicily, false));
    graph.add_city(City::new("Tambrina", Region::Sicily, false));
    graph.add_city(City::new("Messina", Region::Sicily, true));
    graph.add_city(City::new("Solunto", Region::Sicily, false));
    graph.add_city(City::new("Termini Imerese", Region::Sicily, false));
    graph.add_city(City::new("Cefalu", Region::Sicily, false));
    graph.add_city(City::new("Castelbuono", Region::Sicily, false));
    graph.add_city(City::new("Petralia Sottana", Region::Sicily, false));
    graph.add_city(City::new("Nicosia", Region::Sicily, false));
    graph.add_city(City::new("Caronia", Region::Sicily, false));
    graph.add_city(City::new("Patti", Region::Sicily, false));
    graph.add_city(City::new("Barcelona Pozo di Goto", Region::Sicily, false));
    graph.add_city(City::new("Milazzo", Region::Sicily, false));
    graph.add_city(City::new("Castoreale", Region::Sicily, false));
    graph.add_city(City::new("Randanzzo", Region::Sicily, false));
    graph.add_city(City::new("Enna", Region::Sicily, true));
    graph.add_city(City::new("Adrano", Region::Sicily, false));
    graph.add_city(City::new("Licata", Region::Sicily, false));
    graph.add_city(City::new("Caltanissetta", Region::Sicily, true));
    graph.add_city(City::new("Canicatti", Region::Sicily, false));
    graph.add_city(City::new("Agrigento", Region::Sicily, true));
    graph.add_city(City::new("Porto Empedocle", Region::Sicily, false));
    graph.add_city(City::new("Sciacca", Region::Sicily, false));
    graph.add_city(City::new("Ribera", Region::Sicily, false));
    graph.add_city(City::new("Corleone", Region::Sicily, false));
    graph.add_city(City::new("Misilmeri", Region::Sicily, false));
    graph.add_city(City::new("Palermo", Region::Sicily, true));
    graph.add_city(City::new("Monreale", Region::Sicily, false));
    graph.add_city(City::new("Mondello", Region::Sicily, false));
    graph.add_city(City::new("Alcamo", Region::Sicily, false));
    graph.add_city(City::new("Partanna", Region::Sicily, false));
    graph.add_city(City::new("Salemi", Region::Sicily, false));
    graph.add_city(City::new("Caltafimi", Region::Sicily, false));
    graph.add_city(City::new("Segesta", Region::Sicily, false));
    graph.add_city(City::new("Trapani", Region::Sicily, true));
    graph.add_city(City::new("Erice", Region::Sicily, false));
    graph.add_city(City::new("Capo San Vito", Region::Sicily, false));
    graph.add_city(City::new("Castellammare", Region::Sicily, false));
    graph.add_city(City::new("Marsala", Region::Sicily, false));
    graph.add_city(City::new("Castelvetrano", Region::Sicily, false));
    graph.add_city(City::new("Mazara del Vallo", Region::Sicily, false));
    graph.add_city(City::new("Menfi", Region::Sicily, false));
    graph.add_city(City::new("Marinella", Region::Sicily, false));
    graph.add_city(City::new("Selinunte", Region::Sicily, false));
    graph.add_city(City::new("Favignana", Region::Sicily, false));
    graph.add_city(City::new("Tracino", Region::Sicily, false));
    graph.add_city(City::new("Pantelleria", Region::Sicily, false));
    graph.add_city(City::new("Ustica", Region::Sicily, false));
    graph.add_city(City::new("Malfa", Region::Sicily, false));
    graph.add_city(City::new("Lipari", Region::Sicily, false));
    graph.add_city(City::new("Stromboli", Region::Sicily, false));
    graph.add_city(City::new("Reggio Calabria", Region::Calabria, true));
    graph.add_city(City::new("Scilla", Region::Calabria, false));
    graph.add_city(City::new("Palmi", Region::Calabria, false));
    graph.add_city(City::new("Gioia Tauro", Region::Calabria, false));
    graph.add_city(City::new("Cittanova", Region::Calabria, false));
    graph.add_city(City::new("Siderno", Region::Calabria, false));
    graph.add_city(City::new("Bovalino", Region::Calabria, false));
    graph.add_city(City::new("Delianuova", Region::Calabria, false));
    graph.add_city(City::new("Africo", Region::Calabria, false));
    graph.add_city(City::new("Melito", Region::Calabria, false));
    graph.add_city(City::new("Tropea", Region::Calabria, false));
    graph.add_city(City::new("Vibo Valentia", Region::Calabria, true));
    graph.add_city(City::new("Pizzo", Region::Calabria, false));
    graph.add_city(City::new("Dinami", Region::Calabria, false));
    graph.add_city(City::new("Caulonia", Region::Calabria, false));
    graph.add_city(City::new("Soverato", Region::Calabria, false));
    graph.add_city(City::new("Curinga", Region::Calabria, false));
    graph.add_city(City::new("Catanzaro", Region::Calabria, true));
    graph.add_city(City::new("Botricello", Region::Calabria, false));
    graph.add_city(City::new("Crotone", Region::Calabria, true));
    graph.add_city(City::new("Ciro Marina", Region::Calabria, false));
    graph.add_city(City::new("Rossano", Region::Calabria, false));
    graph.add_city(City::new("Strongoli", Region::Calabria, false));
    graph.add_city(City::new("Cutro", Region::Calabria, false));
    graph.add_city(City::new("Longobucco", Region::Calabria, false));
    graph.add_city(City::new("Cotronei", Region::Calabria, false));
    graph.add_city(City::new("Rogliano", Region::Calabria, false));
    graph.add_city(City::new("Acri", Region::Calabria, false));
    graph.add_city(City::new("Cosenza", Region::Calabria, true));
    graph.add_city(City::new("Grimaldi", Region::Calabria, false));
    graph.add_city(City::new("Falerna", Region::Calabria, false));
    graph.add_city(City::new("Paola", Region::Calabria, false));
    graph.add_city(City::new("Scalea", Region::Calabria, false));
    graph.add_city(City::new("Castrovillari", Region::Calabria, false));
    graph.add_city(City::new("Oriolo", Region::Calabria, false));
    graph.add_city(City::new("Roseto", Region::Calabria, false));
    graph.add_city(City::new("Sibari", Region::Calabria, false));
    graph.add_city(City::new("Altomonte", Region::Calabria, false));
    graph.add_city(City::new("Maratea", Region::Basilicata, false));
    graph.add_city(City::new("Lauria", Region::Basilicata, false));
    graph.add_city(City::new("Aliano", Region::Basilicata, false));
    graph.add_city(City::new("Fernandina", Region::Basilicata, false));
    graph.add_city(City::new("Matera", Region::Basilicata, true));
    graph.add_city(City::new("Metaponto", Region::Basilicata, false));
    graph.add_city(City::new("Pisticci", Region::Basilicata, false));
    graph.add_city(City::new("Policoro", Region::Basilicata, false));
    graph.add_city(City::new("Potenza", Region::Basilicata, true));
    graph.add_city(City::new("Brienza", Region::Basilicata, false));
    graph.add_city(City::new("Melfi", Region::Basilicata, false));
    graph.add_city(City::new("Tursi", Region::Basilicata, false));
    graph.add_city(City::new("Napoli", Region::Campania, true));
    graph.add_city(City::new("Ischia", Region::Campania, false));
    graph.add_city(City::new("Sapri", Region::Campania, true));
    graph.add_city(City::new("Camerota", Region::Campania, false));
    graph.add_city(City::new("Ascea", Region::Campania, false));
    graph.add_city(City::new("Padula", Region::Campania, false));
    graph.add_city(City::new("Polla", Region::Campania, false));
    graph.add_city(City::new("Battipaglia", Region::Campania, false));
    graph.add_city(City::new("Agropoli", Region::Campania, false));
    graph.add_city(City::new("Sorrento", Region::Campania, false));
    graph.add_city(City::new("Salerno", Region::Campania, true));
    graph.add_city(City::new("Contursi", Region::Campania, false));
    graph.add_city(City::new("Nola", Region::Campania, false));
    graph.add_city(City::new("Avellino", Region::Campania, true));
    graph.add_city(City::new("Varcaturo", Region::Campania, false));
    graph.add_city(City::new("Mondragone", Region::Campania, false));
    graph.add_city(City::new("Teano", Region::Campania, false));
    graph.add_city(City::new("Alife", Region::Campania, false));
    graph.add_city(City::new("Caserta", Region::Campania, false));
    graph.add_city(City::new("Barba", Region::Campania, false));
    graph.add_city(City::new("Lioni", Region::Campania, false));
    graph.add_city(City::new("Benevento", Region::Campania, true));
    graph.add_city(City::new("Circello", Region::Campania, false));
    graph.add_city(City::new("Romiti", Region::Campania, false));
    graph.add_city(City::new("Calaggoi", Region::Campania, false));
    graph.add_city(City::new("Lucera", Region::Apulia, false));
    graph.add_city(City::new("Troia", Region::Apulia, false));
    graph.add_city(City::new("Catenaccio", Region::Apulia, false));
    graph.add_city(City::new("Cerignola", Region::Apulia, false));
    graph.add_city(City::new("Barletta", Region::Apulia, true));
    graph.add_city(City::new("Foggia", Region::Apulia, true));
    graph.add_city(City::new("Zapponeta", Region::Apulia, false));
    graph.add_city(City::new("Arpinova", Region::Apulia, false));
    graph.add_city(City::new("Manfredonia", Region::Apulia, false));
    graph.add_city(City::new("Mattinata", Region::Apulia, false));
    graph.add_city(City::new("Vieste", Region::Apulia, false));
    graph.add_city(City::new("Peschici", Region::Apulia, false));
    graph.add_city(City::new("Carpino", Region::Apulia, false));
    graph.add_city(City::new("San Severo", Region::Apulia, false));
    graph.add_city(City::new("Chieuti", Region::Apulia, false));
    graph.add_city(City::new("Lesina", Region::Apulia, false));
    graph.add_city(City::new("Andria", Region::Apulia, false));
    graph.add_city(City::new("Trani", Region::Apulia, false));
    graph.add_city(City::new("Corrato", Region::Apulia, false));
    graph.add_city(City::new("Molfetta", Region::Apulia, false));
    graph.add_city(City::new("Torrito", Region::Apulia, false));
    graph.add_city(City::new("Altamura", Region::Apulia, false));
    graph.add_city(City::new("Cicerone", Region::Apulia, false));
    graph.add_city(City::new("Bari", Region::Apulia, true));
    graph.add_city(City::new("Triggiano", Region::Apulia, false));
    graph.add_city(City::new("Monopoli", Region::Apulia, false));
    graph.add_city(City::new("Ostuni", Region::Apulia, false));
    graph.add_city(City::new("Fasano", Region::Apulia, false));
    graph.add_city(City::new("Ginosa", Region::Apulia, false));
    graph.add_city(City::new("Chiatona", Region::Apulia, false));
    graph.add_city(City::new("Taranto", Region::Apulia, true));
    graph.add_city(City::new("Lissano", Region::Apulia, false));
    graph.add_city(City::new("Librari", Region::Apulia, false));
    graph.add_city(City::new("Motola", Region::Apulia, false));
    graph.add_city(City::new("Brindisi", Region::Apulia, true));
    graph.add_city(City::new("Mesagne", Region::Apulia, false));
    graph.add_city(City::new("Casalabate", Region::Apulia, false));
    graph.add_city(City::new("Urmo", Region::Apulia, false));
    graph.add_city(City::new("Lecce", Region::Apulia, true));
    graph.add_city(City::new("Gallipoli", Region::Apulia, false));
    graph.add_city(City::new("Galatina", Region::Apulia, false));
    graph.add_city(City::new("Otranto", Region::Apulia, false));
    graph.add_city(City::new("Tricase", Region::Apulia, false));
    graph.add_city(City::new("Santa Maria di Leuca", Region::Apulia, false));
    graph.add_city(City::new("Venafro", Region::Molise, false));
    graph.add_city(City::new("Isernia", Region::Molise, false));
    graph.add_city(City::new("Agnone", Region::Molise, false));
    graph.add_city(City::new("Oratina", Region::Molise, false));
    graph.add_city(City::new("Campobasso", Region::Molise, true));
    graph.add_city(City::new("Lucito", Region::Molise, false));
    graph.add_city(City::new("Larino", Region::Molise, false));
    graph.add_city(City::new("Termoli", Region::Molise, false));
    graph.add_city(City::new("Palata", Region::Molise, false));


    // Adding connections to the graph
    //God bless the Queen
    // Adding connections to the graph
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Marina di Ragusa"),
        33,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Modica"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Comiso"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Ragusa", "Giarratana"),
        17,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Comiso", "Vittoria"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Vittoria", "Gela"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Marina di Ragusa", "Pozzallo"),
        31,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Pozzallo", "Modica"),
        30,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Pozzallo", "Noto Marioa"),
        41,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Pozzallo", "Capo Passero"),
        40,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Modica", "Noto Marioa"),
        41,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Modica", "Capo Passero"),
        40,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Capo Passero", "Noto Marioa"),
        36,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Noto Marioa", "Siracusa"),
        31,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Siracusa", "Floridia"),
        14,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Siracusa", "Augusta"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Augusta", "Lentini"),
        24,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lentini", "Catania"),
        31,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Floridia", "Palazzolo Acreide"),
        18,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Palazzolo Acreide", "Giarratana"),
        12,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Palazzolo Acreide", "Caltagirone"),
        48,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Gela", "Caltagirone"),
        37,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Gela", "Enna"),
        76,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Gela", "Licata"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltagirone", "Gerbini"),
        48,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltagirone", "Enna"),
        50,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Gerbini", "Paterno"),
        10,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Paterno", "Catania"),
        11,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Paterno", "Adrano"),
        9,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Catania", "Acireale"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Catania", "Messina"),
        92,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Acireale", "Riposte"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Riposte", "Tambrina"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Tambrina", "Messina"),
        37,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Messina", "Milazzo"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Messina", "Reggio Calabria"),
        13,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Solunto", "Termini Imerese"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Solunto", "Palermo"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Termini Imerese", "Milazzo"),
        170,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Termini Imerese", "Cefalu"),
        54,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Termini Imerese", "Petralia Sottana"),
        61,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Cefalu", "Petralia Sottana"),
        60,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Cefalu", "Castelbuono"),
        7,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Cefalu", "Caronia"),
        39,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Castelbuono", "Petralia Sottana"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Petralia Sottana", "Nicosia"),
        18,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Nicosia", "Enna"),
        29,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Caronia", "Patti"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Patti", "Castoreale"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Barcelona Pozo di Goto", "Milazzo"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Barcelona Pozo di Goto", "Castoreale"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Barcelona Pozo di Goto", "Patti"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Milazzo", "Lipari"),
        44,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Castoreale", "Randanzzo"),
        33,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Randanzzo", "Adrano"),
        48,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Enna", "Adrano"),
        28,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Enna", "Caltanissetta"),
        24,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Licata", "Caltanissetta"),
        54,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Licata", "Canicatti"),
        34,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Licata", "Agrigento"),
        54,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltanissetta", "Canicatti"),
        34,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltanissetta", "Misilmeri"),
        93,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Canicatti", "Agrigento"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Agrigento", "Porto Empedocle"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Agrigento", "Ribera"),
        36,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Porto Empedocle", "Sciacca"),
        35,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Sciacca", "Menfi"),
        23,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ribera", "Corleone"),
        39,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Corleone", "Misilmeri"),
        40,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Corleone", "Partanna"),
        63,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Misilmeri", "Palermo"),
        21,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Palermo", "Monreale"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Palermo", "Alcamo"),
        46,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Palermo", "Mondello"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Monreale", "Alcamo"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Monreale", "Mondello"),
        50,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Mondello", "Alcamo"),
        39,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Castellammare"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Segesta"),
        19,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Salemi"),
        34,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Partanna"),
        41,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Castelvetrano"),
        46,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Alcamo", "Caltafimi"),
        20,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Partanna", "Castelvetrano"),
        30,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Partanna", "Salemi"),
        26,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Partanna", "Caltafimi"),
        48,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Partanna", "Segesta"),
        48,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Salemi", "Castelvetrano"),
        33,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Salemi", "Caltafimi"),
        42,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Salemi", "Segesta"),
        42,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltafimi", "Segesta"),
        14,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Caltafimi", "Castelvetrano"),
        55,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Segesta", "Trapani"),
        25,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Segesta", "Castelvetrano"),
        55,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Trapani", "Marsala"),
        38,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Trapani", "Erice"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Trapani", "Pantelleria"),
        110,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Trapani", "Favignana"),
        22,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Erice", "Capo San Vito"),
        23,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Erice", "Castellammare"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Capo San Vito", "Castellammare"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Capo San Vito", "Ustica"),
        63,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Marsala", "Mazara del Vallo"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Castelvetrano", "Mazara del Vallo"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Castelvetrano", "Marinella"),
        29,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Castelvetrano", "Menfi"),
        31,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Mazara del Vallo", "Menfi"),
        51,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Mazara del Vallo", "Marinella"),
        42,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Menfi", "Marinella"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Marinella", "Selinunte"),
        15,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Tracino", "Pantelleria"),
        9,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Ustica", "Malfa"),
        78,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Malfa", "Lipari"),
        9,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Lipari", "Stromboli"),
        31,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Stromboli", "Napoli"),
        160,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Reggio Calabria", "Melito"),
        20,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Reggio Calabria", "Scilla"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Scilla", "Palmi"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Palmi", "Delianuova"),
        11,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Palmi", "Gioia Tauro"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Gioia Tauro", "Cittanova"),
        19,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Gioia Tauro", "Tropea"),
        27,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Cittanova", "Siderno"),
        14,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Cittanova", "Dinami"),
        18,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Siderno", "Bovalino"),
        9,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Siderno", "Caulonia"),
        29,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Bovalino", "Africo"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Bovalino", "Delianuova"),
        21,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Africo", "Melito"),
        43,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Tropea", "Vibo Valentia"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Vibo Valentia", "Pizzo"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Vibo Valentia", "Dinami"),
        20,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Vibo Valentia", "Soverato"),
        38,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Pizzo", "Curinga"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Caulonia", "Soverato"),
        35,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Soverato", "Curinga"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Soverato", "Catanzaro"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Curinga", "Falerna"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Curinga", "Catanzaro"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Catanzaro", "Botricello"),
        20,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Catanzaro", "Cutro"),
        31,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Botricello", "Crotone"),
        25,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Crotone", "Cutro"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Crotone", "Ciro Marina"),
        31,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Crotone", "Santa Maria di Leuca"),
        90,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Ciro Marina", "Rossano"),
        41,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Ciro Marina", "Strongoli"),
        8,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Rossano", "Longobucco"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Rossano", "Cosenza"),
        38,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Rossano", "Sibari"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Strongoli", "Longobucco"),
        27,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Strongoli", "Cutro"),
        16,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Cutro", "Cotronei"),
        19,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Longobucco", "Acri"),
        16,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Cotronei", "Rogliano"),
        22,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Rogliano", "Cosenza"),
        17,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Acri", "Cosenza"),
        28,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Cosenza", "Grimaldi"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Cosenza", "Paola"),
        17,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Cosenza", "Altomonte"),
        32,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Grimaldi", "Falerna"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Falerna", "Paola"),
        47,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Paola", "Scalea"),
        46,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Scalea", "Maratea"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Castrovillari", "Scalea"),
        31,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Castrovillari", "Altomonte"),
        19,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Castrovillari", "Oriolo"),
        26,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Oriolo", "Roseto"),
        14,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Oriolo", "Lauria"),
        30,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Oriolo", "Tursi"),
        15,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Roseto", "Sibari"),
        27,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Roseto", "Policoro"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Sibari", "Altomonte"),
        23,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Maratea", "Lauria"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lauria", "Tursi"),
        30,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Lauria", "Aliano"),
        26,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Lauria", "Brienza"),
        54,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Aliano", "Brienza"),
        42,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Aliano", "Fernandina"),
        15,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Aliano", "Tursi"),
        16,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Fernandina", "Matera"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Fernandina", "Potenza"),
        27,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Fernandina", "Pisticci"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Matera", "Potenza"),
        36,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Matera", "Metaponto"),
        61,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Matera", "Altamura"),
        18,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Metaponto", "Pisticci"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Metaponto", "Policoro"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Metaponto", "Ginosa"),
        8,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Pisticci", "Tursi"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Policoro", "Tursi"),
        15,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Potenza", "Melfi"),
        25,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Potenza", "Brienza"),
        25,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Brienza", "Melfi"),
        25,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Brienza", "Polla"),
        19,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Melfi", "Lioni"),
        25,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Melfi", "Andria"),
        42,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Napoli", "Salerno"),
        31,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Napoli", "Nola"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Napoli", "Varcaturo"),
        17,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Napoli", "Caserta"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Napoli", "Ischia"),
        20,
        RoadTypes::Ferry,
    ));
    graph.add_connection(CityConnection::new(
        ("Sapri", "Maratea"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Sapri", "Camerota"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Sapri", "Padula"),
        25,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Camerota", "Ascea"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ascea", "Agropoli"),
        20,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Padula", "Agropoli"),
        26,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Padula", "Polla"),
        16,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Polla", "Battipaglia"),
        24,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Polla", "Contursi"),
        12,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Battipaglia", "Salerno"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Battipaglia", "Agropoli"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Sorrento", "Salerno"),
        24,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Salerno", "Avellino"),
        17,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Salerno", "Contursi"),
        23,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Contursi", "Lioni"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Nola", "Avellino"),
        22,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Avellino", "Benevento"),
        24,
        RoadTypes::Highway,
    ));
    graph.add_connection(CityConnection::new(
        ("Varcaturo", "Mondragone"),
        27,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Mondragone", "Teano"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Teano", "Caserta"),
        25,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Teano", "Alife"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Teano", "Venafro"),
        14,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Alife", "Benevento"),
        30,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Alife", "Caserta"),
        26,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Alife", "Circello"),
        32,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Caserta", "Barba"),
        31,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Barba", "Benevento"),
        7,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Lioni", "Benevento"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lioni", "Calaggoi"),
        10,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Benevento", "Circello"),
        12,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Benevento", "Romiti"),
        12,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Circello", "Campobasso"),
        34,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Circello", "Romiti"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Romiti", "Calaggoi"),
        25,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Romiti", "Lucera"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Calaggoi", "Catenaccio"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lucera", "Foggia"),
        20,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lucera", "Campobasso"),
        40,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lucera", "Troia"),
        13,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Lucera", "San Severo"),
        14,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Troia", "Catenaccio"),
        17,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Catenaccio", "Cerignola"),
        19,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Cerignola", "Foggia"),
        32,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Cerignola", "Barletta"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Cerignola", "Andria"),
        15,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Barletta", "Trani"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Barletta", "Andria"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Barletta", "Zapponeta"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Foggia", "Zapponeta"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Foggia", "Arpinova"),
        8,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Zapponeta", "Manfredonia"),
        19,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Arpinova", "Manfredonia"),
        17,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Manfredonia", "Mattinata"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Manfredonia", "Carpino"),
        20,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Manfredonia", "Vieste"),
        24,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Mattinata", "Vieste"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Vieste", "Peschici"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Vieste", "Carpino"),
        25,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Peschici", "Carpino"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Carpino", "San Severo"),
        21,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Carpino", "Chieuti"),
        28,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("San Severo", "Chieuti"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Chieuti", "Lesina"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Chieuti", "Larino"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Chieuti", "Termoli"),
        23,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Andria", "Corrato"),
        9,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Trani", "Molfetta"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Corrato", "Molfetta"),
        14,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Corrato", "Torrito"),
        21,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Molfetta", "Bari"),
        12,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Molfetta", "Torrito"),
        15,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Torrito", "Bari"),
        11,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Torrito", "Altamura"),
        26,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Torrito", "Cicerone"),
        37,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Altamura", "Cicerone"),
        34,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Cicerone", "Motola"),
        20,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Cicerone", "Fasano"),
        21,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Bari", "Fasano"),
        46,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Bari", "Triggiano"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Triggiano", "Monopoli"),
        24,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Monopoli", "Ostuni"),
        18,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ostuni", "Brindisi"),
        20,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Ostuni", "Fasano"),
        18,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Ostuni", "Mesagne"),
        21,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Fasano", "Motola"),
        19,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Ginosa", "Chiatona"),
        16,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Chiatona", "Taranto"),
        10,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Taranto", "Motola"),
        15,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Taranto", "Lissano"),
        13,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Taranto", "Librari"),
        21,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lissano", "Motola"),
        17,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Lissano", "Librari"),
        14,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Lissano", "Mesagne"),
        26,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Librari", "Urmo"),
        20,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Brindisi", "Mesagne"),
        11,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Brindisi", "Casalabate"),
        22,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Mesagne", "Lecce"),
        39,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Casalabate", "Lecce"),
        10,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Urmo", "Lecce"),
        37,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Urmo", "Gallipoli"),
        38,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lecce", "Galatina"),
        28,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Lecce", "Otranto"),
        30,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Gallipoli", "Galatina"),
        18,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Galatina", "Otranto"),
        17,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Galatina", "Tricase"),
        23,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Otranto", "Tricase"),
        23,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Tricase", "Santa Maria di Leuca"),
        7,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Isernia", "Oratina"),
        22,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Agnone", "Oratina"),
        26,
        RoadTypes::Unpaved,
    ));
    graph.add_connection(CityConnection::new(
        ("Agnone", "Lucito"),
        37,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Oratina", "Campobasso"),
        8,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Campobasso", "Lucito"),
        13,
        RoadTypes::Asphalt,
    ));
    graph.add_connection(CityConnection::new(
        ("Lucito", "Larino"),
        12,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Larino", "Palata"),
        11,
        RoadTypes::Cobblestone,
    ));
    graph.add_connection(CityConnection::new(
        ("Termoli", "Palata"),
        11,
        RoadTypes::Cobblestone,
    ));



    graph
}
