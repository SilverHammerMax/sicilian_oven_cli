use crate::types::*;
use rand;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Clone)]
pub enum Location {
    City(String),
    Region(city::Region),
    Any,
}

#[derive(Clone)]
pub struct Challenge {
    name: String,
    car: Option<car_parts::car::Car>,
    cities: Vec<String>,
    start_city: Location,
    end_city: Location,
    medal_cutoffs: Option<[i32; 4]>,
    medal: medal::Medal,
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.medal {
            medal::Medal::None => write!(f, "{}", self.name),
            _ => write!(f, "{} {}", self.name, self.medal),
        }
    }
}

impl Challenge {
    pub fn new(
        name: &str,
        car: Option<car_parts::car::Car>,
        cities: Vec<&str>,
        start_city: Location,
        end_city: Location,
        medal_cutoffs: Option<[i32; 4]>,
    ) -> Challenge {
        Challenge {
            name: name.to_string(),
            car,
            cities: cities.iter().map(|city| city.to_string()).collect(),
            start_city,
            end_city,
            medal_cutoffs,
            medal: medal::Medal::None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn car(&self) -> Option<car_parts::car::Car> {
        self.car.clone()
    }

    pub fn cities(&self) -> &[String] {
        self.cities.as_slice()
    }

    pub fn start_city(&self) -> &Location {
        &self.start_city
    }

    pub fn get_end_city(&self) -> &Location {
        &self.end_city
    }

    pub fn medal_cutoffs(&self) -> Option<[i32; 4]> {
        self.medal_cutoffs
    }

    pub fn get_medal(&self) -> &medal::Medal {
        &self.medal
    }

    pub fn set_medal(&mut self, medal: medal::Medal) {
        self.medal = medal;
    }
}

pub fn initialize_challenges() -> Vec<Challenge> {
    let cars = car_parts::car::Car::initialize();
    vec![
        Challenge::new(
            "Ragusan Ride",
            Some(cars[3].clone()),
            vec![
                "Ragusa",
                "Comiso",
                "Vittoria",
                "Marina di Ragusa",
                "Modica",
                "Pozzallo",
                "Capo Passero",
                "Noto Marioa",
                "Siracusa",
                "Giarratana",
                "Palazzolo Acreide",
                "Floridia",
                "Augusta",
                "Lentini",
            ],
            Location::City("Ragusa".to_string()),
            Location::City("Ragusa".to_string()),
            Some([205, 220, 270, 330]),
        ),
        Challenge::new(
            "Big Car, Big Cities",
            Some(cars[0].clone()),
            vec![
                "Ragusa",
                "Siracusa",
                "Catania",
                "Enna",
                "Caltanissetta",
                "Palermo",
                "Trapani",
                "Messina",
                "Agrigento",
            ],
            Location::Region(city::Region::Sicily),
            Location::Region(city::Region::Sicily),
            Some([310, 325, 375, 475]),
        ),
        Challenge::new(
            "A Ride Around Mt. Etna",
            Some(cars[2].clone()),
            vec![
                "Catania",
                "Gerbini",
                "Paterno",
                "Adrano",
                "Randanzzo",
                "Castoreale",
                "Patti",
                "Barcelona Pozo di Goto",
                "Milazzo",
                "Messina",
                "Riposte",
                "Tambrina",
                "Acireale",
                "Lentini",
                "Nicosia",
                "Enna",
            ],
            Location::City("Catania".to_string()),
            Location::City("Catania".to_string()),
            Some([290, 310, 335, 395]),
        ),
        Challenge::new(
            "The Godfather",
            Some(cars[3].clone()),
            vec![
                "Corleone",
                "Selinunte",
                "Marinella",
                "Castelvetrano",
                "Partanna",
                "Menfi",
                "Sciacca",
                "Porto Empedocle",
                "Agrigento",
                "Ribera",
                "Canicatti",
                "Licata",
            ],
            Location::City("Corleone".to_string()),
            Location::City("Corleone".to_string()),
            Some([305, 325, 370, 395]),
        ),
        Challenge::new(
            "Harbormaster",
            None,
            vec![
                "Ischia",
                "Strongoli",
                "Lipari",
                "Malfa",
                "Ustica",
                "Favignana",
                "Pantelleria",
                "Tricase",
            ],
            Location::Any,
            Location::Any,
            Some([0, 0, 0, 0]),
        ),
        Challenge::new(
            "A Calabrian Rally",
            Some(cars[1].clone()),
            vec![
                "Acri",
                "Cotronei",
                "Oriolo",
                "Dinami",
                "Delianuova",
            ],
            Location::City("Catanzaro".to_string()),
            Location::Any,
            Some([0, 0, 0, 0]),
        ),
        Challenge::new(
            "Free Play",
            None,
            vec![],
            Location::Any,
            Location::Any,
            None,
        ),
    ]
}

pub fn random_challenge(cities: &crate::cities::CityGraph) -> Challenge {
    let count = dialoguer::Input::new()
        .with_prompt("How many cities would you like to go to?")
        .with_initial_text("5")
        .validate_with(|input: &i32| -> Result<(), &str> {
            if *input <= 0 {
                return Err("Too Few Cities");
            }
            if *input > cities.cities().len() as i32 {
                return Err("Too Many Cities");
            }
            Ok(())
        })
        .interact_text()
        .expect("Prompt Failed") as usize;

    let seed: String = dialoguer::Input::new()
        .with_prompt("What seed would you like to use (Leave blank for a random seed)?")
        .allow_empty(true)
        .interact_text()
        .expect("Prompt Failed");

    let seed = match seed.as_str() {
        "" => thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(256)
            .map(char::from)
            .collect(),
        _ => seed,
    };

    let mut rng: Pcg64 = Seeder::from(seed.to_owned() + &count.to_string()).make_rng();
    let challenge_cities = cities
        .cities()
        .iter()
        .map(|city| city.get_name())
        .choose_multiple(&mut rng, count);
    Challenge::new(
        "Random Cities",
        None,
        challenge_cities,
        Location::Any,
        Location::Any,
        None,
    )
}
