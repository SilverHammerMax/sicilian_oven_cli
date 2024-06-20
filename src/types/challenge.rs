use rand;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use crate::types::*;

#[derive(PartialEq, Eq, Clone)]
pub enum Location {
    City(&'static str),
    Region(city::Region),
    Any,
}

#[derive(Clone)]
pub struct Challenge {
    name: &'static str,
    car: Option<car_parts::car::Car>,
    cities: Vec<&'static str>,
    start_city: Location,
    end_city: Location,
    medal_cutoffs: [i32; 4],
    medal: medal::Medal,
}

impl Challenge {
    pub fn new(
        name: &'static str,
        car: Option<car_parts::car::Car>,
        cities: Vec<&'static str>,
        start_city: Location,
        end_city: Location,
        medal_cutoffs: [i32; 4],
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
        self.car
    }

    pub fn cities(&self) -> &[&str] {
        self.cities.as_slice()
    }

    pub fn start_city(&self) -> &Location {
        &self.start_city
    }

    pub fn get_end_city(&self) -> &Location {
        &self.end_city
    }

    pub fn medal_cutoffs(&self) -> [i32; 4] {
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
    let cars = car_parts::car::initialize_cars();
    vec![
        Challenge::new(
            "Ragusan Ride",
            Some(cars[3]),
            vec![
                "RAG", "COM", "VIT", "MDR", "MOD", "POZ", "CAP", "NTO", "SIR", "GIA", "PAL", "FLO",
                "AUG", "LEN",
            ],
            Location::City("RAG"),
            Location::City("RAG"),
            [205, 220, 270, 330],
        ),
        Challenge::new(
            "Big Car, Big Cities",
            Some(cars[0]),
            vec![
                "RAG", "SIR", "CAT", "ENN", "CTN", "PMO", "TRA", "MES", "AGR",
            ],
            Location::Region(city::Region::Sicily),
            Location::Region(city::Region::Sicily),
            [310, 325, 375, 475],
        ),
        Challenge::new(
            "A Ride Around Mt. Etna",
            Some(cars[2]),
            vec![
                "CAT", "GER", "PAT", "ADR", "RAN", "CRL", "PTI", "BAR", "MIL", "MES", "RIP", "TAM",
                "ACI", "LEN", "NIC", "ENN",
            ],
            Location::City("CAT"),
            Location::City("CAT"),
            [290, 310, 335, 395],
        ),
        Challenge::new(
            "The Godfather",
            Some(cars[3]),
            vec![
                "COR", "SEL", "MAR", "CST", "PAR", "MEN", "SCI", "POR", "AGR", "RIB", "CAN", "LIC",
            ],
            Location::City("COR"),
            Location::City("COR"),
            [305, 325, 370, 395],
        ),
        Challenge::new(
            "Harbormaster",
            None,
            vec![
                "ISC", "STR", "LIP", "MAL", "FAV", "PAN", "TRC"
            ],
            Location::Any,
            Location::Any,
            [0, 0, 0, 0],
        ),
        Challenge::new(
            "A Calabrian Rally",
            Some(cars[1]),
            vec![
                "ACR", "CTE", "ORI", "DIN", "DNV"
            ],
            Location::City("CNZ"),
            Location::Any,
            [0, 0, 0, 0],
        ),
        Challenge::new(
            "Free Play",
            None,
            vec![],
            Location::Any,
            Location::Any,
            [0, 0, 0, 0],
        ),
    ]
}

pub fn random_challenge() -> Challenge {
    let count: usize = dialoguer::Input::new()
        .with_prompt("How many cities would you like to go to?")
        .with_initial_text("5")
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input < 0 {
                return Err("Too Few Cities")
            }
            if *input > crate::cities::CITIES.len() {
                return Err("Too Many Cities")
            }
            Ok(())
        })
        .interact_text()
        .expect("Prompt Failed");

    let mut seed: String = dialoguer::Input::new()
        .with_prompt("What seed would you like to use (Leave blank for a random seed)?")
        .allow_empty(true)
        .interact_text()
        .expect("Prompt Failed");

    let seed = match seed.as_str() {
        "" => thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(20).map(char::from).collect(),
        _ => seed
    };

    if count > crate::cities::CITIES.len() {
        panic!("Too Many Cities!");
    }
    let mut rng: Pcg64 = Seeder::from(seed.to_owned() + count.to_string().as_str()).make_rng();
    let cities = crate::cities::CITIES.keys().map(|code| *code).choose_multiple(&mut rng, count);
    Challenge::new("Random Cities", None, cities, Location::Any, Location::Any, [0, 0, 0, 0])
}