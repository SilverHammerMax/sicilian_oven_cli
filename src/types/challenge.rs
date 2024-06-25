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
                "RAG".to_string(),
                "COM".to_string(),
                "VIT".to_string(),
                "MDR".to_string(),
                "MOD".to_string(),
                "POZ".to_string(),
                "CAP".to_string(),
                "NTO".to_string(),
                "SIR".to_string(),
                "GIA".to_string(),
                "PAL".to_string(),
                "FLO".to_string(),
                "AUG".to_string(),
                "LEN".to_string(),
            ],
            Location::City("RAG".to_string()),
            Location::City("RAG".to_string()),
            Some([205, 220, 270, 330]),
        ),
        Challenge::new(
            "Big Car, Big Cities",
            Some(cars[0].clone()),
            vec![
                "RAG".to_string(),
                "SIR".to_string(),
                "CAT".to_string(),
                "ENN".to_string(),
                "CTN".to_string(),
                "PMO".to_string(),
                "TRA".to_string(),
                "MES".to_string(),
                "AGR".to_string(),
            ],
            Location::Region(city::Region::Sicily),
            Location::Region(city::Region::Sicily),
            Some([310, 325, 375, 475]),
        ),
        Challenge::new(
            "A Ride Around Mt. Etna",
            Some(cars[2].clone()),
            vec![
                "CAT".to_string(),
                "GER".to_string(),
                "PAT".to_string(),
                "ADR".to_string(),
                "RAN".to_string(),
                "CRL".to_string(),
                "PTI".to_string(),
                "BAR".to_string(),
                "MIL".to_string(),
                "MES".to_string(),
                "RIP".to_string(),
                "TAM".to_string(),
                "ACI".to_string(),
                "LEN".to_string(),
                "NIC".to_string(),
                "ENN".to_string(),
            ],
            Location::City("CAT".to_string()),
            Location::City("CAT".to_string()),
            Some([290, 310, 335, 395]),
        ),
        Challenge::new(
            "The Godfather",
            Some(cars[3].clone()),
            vec![
                "COR".to_string(),
                "SEL".to_string(),
                "MAR".to_string(),
                "CST".to_string(),
                "PAR".to_string(),
                "MEN".to_string(),
                "SCI".to_string(),
                "POR".to_string(),
                "AGR".to_string(),
                "RIB".to_string(),
                "CAN".to_string(),
                "LIC".to_string(),
            ],
            Location::City("COR".to_string()),
            Location::City("COR".to_string()),
            Some([305, 325, 370, 395]),
        ),
        Challenge::new(
            "Harbormaster",
            None,
            vec![
                "ISC".to_string(),
                "STR".to_string(),
                "LIP".to_string(),
                "MAL".to_string(),
                "FAV".to_string(),
                "PAN".to_string(),
                "TRC".to_string(),
            ],
            Location::Any,
            Location::Any,
            Some([0, 0, 0, 0]),
        ),
        Challenge::new(
            "A Calabrian Rally",
            Some(cars[1].clone()),
            vec![
                "ACR".to_string(),
                "CTE".to_string(),
                "ORI".to_string(),
                "DIN".to_string(),
                "DNV".to_string(),
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
