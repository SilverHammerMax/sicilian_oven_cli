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
    name: &'static str,
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
        name: &'static str,
        car: Option<car_parts::car::Car>,
        cities: Vec<String>,
        start_city: Location,
        end_city: Location,
        medal_cutoffs: Option<[i32; 4]>,
    ) -> Challenge {
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

    pub fn name(&self) -> &str {
        self.name
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
                "Ragusa".to_string(),
                "Comiso".to_string(),
                "Vittoria".to_string(),
                "Marina di Ragusa".to_string(),
                "Modica".to_string(),
                "Pozzallo".to_string(),
                "Capo Passero".to_string(),
                "Noto Marioa".to_string(),
                "Siracusa".to_string(),
                "Giarratana".to_string(),
                "Palazzolo Acreide".to_string(),
                "Floridia".to_string(),
                "Augusta".to_string(),
                "Lentini".to_string(),
            ],
            Location::City("Ragusa".to_string()),
            Location::City("Ragusa".to_string()),
            Some([205, 220, 270, 330]),
        ),
        Challenge::new(
            "Big Car, Big Cities",
            Some(cars[0].clone()),
            vec![
                "Ragusa".to_string(),
                "Siracusa".to_string(),
                "Catania".to_string(),
                "Enna".to_string(),
                "Caltanissetta".to_string(),
                "Palermo".to_string(),
                "Trapani".to_string(),
                "Messina".to_string(),
                "Agrigento".to_string(),
            ],
            Location::Region(city::Region::Sicily),
            Location::Region(city::Region::Sicily),
            Some([310, 325, 375, 475]),
        ),
        Challenge::new(
            "A Ride Around Mt. Etna",
            Some(cars[2].clone()),
            vec![
                "Catania".to_string(),
                "Gerbini".to_string(),
                "Paterno".to_string(),
                "Adrano".to_string(),
                "Randanzzo".to_string(),
                "Castoreale".to_string(),
                "Patti".to_string(),
                "Barcelona Pozo di Goto".to_string(),
                "Milazzo".to_string(),
                "Messina".to_string(),
                "Riposte".to_string(),
                "Tambrina".to_string(),
                "Acireale".to_string(),
                "Lentini".to_string(),
                "Nicosia".to_string(),
                "Enna".to_string(),
            ],
            Location::City("Catania".to_string()),
            Location::City("Catania".to_string()),
            Some([290, 310, 335, 395]),
        ),
        Challenge::new(
            "The Godfather",
            Some(cars[3].clone()),
            vec![
                "Corleone".to_string(),
                "Selinunte".to_string(),
                "Marinella".to_string(),
                "Castellammare".to_string(),
                "Partanna".to_string(),
                "Menfi".to_string(),
                "Sciacca".to_string(),
                "Porto Emedocle".to_string(),
                "Agrigento".to_string(),
                "Ribera".to_string(),
                "Canicatti".to_string(),
                "Licata".to_string(),
            ],
            Location::City("Corleone".to_string()),
            Location::City("Corleone".to_string()),
            Some([305, 325, 370, 395]),
        ),
        Challenge::new(
            "Harbormaster",
            None,
            vec![
                "Ischia".to_string(),
                "Strongoli".to_string(),
                "Lipari".to_string(),
                "Malfa".to_string(),
                "Faviganana".to_string(),
                "Pantelleria".to_string(),
                "Tricase".to_string(),
            ],
            Location::Any,
            Location::Any,
            Some([0, 0, 0, 0]),
        ),
        Challenge::new(
            "A Calabrian Rally",
            Some(cars[1].clone()),
            vec![
                "Acri".to_string(),
                "Cotronei".to_string(),
                "Oriolo".to_string(),
                "Dinami".to_string(),
                "Delianuova".to_string(),
            ],
            Location::City("CNZ".to_string()),
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
        .map(|city| city.get_name().to_string())
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
