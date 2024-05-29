use crate::types::*;
use crate::*;
use strum::IntoEnumIterator;

pub fn choose_car() -> car::Car {
    let mut car_names = Vec::new();
    for car in car::CarType::iter() {
        car_names.push(format!("{}", car.get_name()));
    }
    car_names.pop();
        let selection = dialoguer::Select::new()
        .with_prompt("Pick your car")
        .items(&car_names)
        .interact()
        .expect("Prompt Failed");

    match selection {
        0 => car::Car::new(car::CarType::Lancia),
        1 => car::Car::new(car::CarType::Maserati),
        2 => car::Car::new(car::CarType::Ferrari),
        3 => car::Car::new(car::CarType::Fiat),
        _ => panic!("Fix Added Car!")
    }
}

pub fn choose_challenge(medals: &HashMap<challenge::Challenge, medal::Medal>) -> challenge::Challenge {
    let mut challenge_names = Vec::new();
    for challenge in challenge::Challenge::iter() {
        challenge_names.push(format!("{} {}", challenge.get_name(), medals.get(&challenge).expect("Challenge Not Found")));
    }
    let selection = dialoguer::Select::new()
        .with_prompt("Please Select a Challenge")
        .items(&challenge_names)
        .interact()
        .expect("Prompt Failed");

    match selection {
        0 => challenge::Challenge::RagusanRide,
        1 => challenge::Challenge::BigCarBigCities,
        2 => challenge::Challenge::ARideAroundMountEtna,
        3 => challenge::Challenge::TheGodfather,
        4 => challenge::Challenge::FreePlay,
        _ => panic!("Fix Added Challenge!")
    }
}

pub fn choose_major_city() -> &'static str {
    let mut major_names = Vec::new();
    for code in constants::MAJOR_CITIES {
        major_names.push(cities::CITIES.get(code).expect("Invalid City Code").get_name());
    }
    let selection = dialoguer::Select::new()
        .with_prompt("What major city would you like to start in?")
        .items(&major_names)
        .interact()
        .expect("Prompt Failed");

    constants::MAJOR_CITIES[selection]
}

pub fn time_multiplier(current_city: &str, next_city: &str) -> f64 {
    if constants::DANGEROUS_PATHS.contains(&(current_city, next_city)) || constants::DANGEROUS_PATHS.contains(&(next_city, current_city)) {
        return 2.0
    }
    if constants::HIGHWAYS.contains(&(current_city, next_city)) || constants::HIGHWAYS.contains(&(next_city, current_city)) {
        return 0.5
    }
    1.0
}

pub fn challenge_prompt(challenge: &challenge::Challenge) {
    println!(
        "\nWelcome to {}! In this challenge, you will attempt to reach the cities of:\n",
        challenge.get_name()
    );
    for city in challenge.get_cities() {
        println!(
            "- {}",
            cities::CITIES
                .get(city)
                .expect("Invalid City Code")
                .get_name()
        );
    }
    println!();

    let start_city = challenge.get_starting_city();
    let end_city = challenge.get_ending_city();

    match start_city {
        Some(code) => println!(
            "You will start in {}.",
            cities::CITIES
                .get(code)
                .expect("Invalid City Code")
                .get_name()
        ),
        None => println!("You can start in whatever major city you prefer."),
    }

    match end_city {
        Some(code) => println!(
            "You will start in {}.",
            cities::CITIES
                .get(code)
                .expect("Invalid City Code")
                .get_name()
        ),
        None => println!("You can start in whatever major city you prefer."),
    }

    println!();

    match challenge.get_car() {
        Some(car) => println!("You are using the {}.", car.get_car_type().get_name()),
        None => println!("You can use whatever car you prefer.")
    }

    println!();
    if challenge != &challenge::Challenge::FreePlay {
        let author_medal = constants::MEDAL_CUTOFFS
            .get(&challenge)
            .expect("Invalid Challenge")[0];
        let gold_medal = constants::MEDAL_CUTOFFS
            .get(&challenge)
            .expect("Invalid Challenge")[1];
        let silver_medal = constants::MEDAL_CUTOFFS
            .get(&challenge)
            .expect("Invalid Challenge")[2];
        let bronze_medal = constants::MEDAL_CUTOFFS
            .get(&challenge)
            .expect("Invalid Challenge")[3];

        println!(
            "Author: {} hours, {} minutes",
            author_medal / 60,
            author_medal % 60
        );
        println!(
            "Gold: {} hours, {} minutes",
            gold_medal / 60,
            gold_medal % 60
        );
        println!(
            "Silver: {} hours, {} minutes",
            silver_medal / 60,
            silver_medal % 60
        );
        println!(
            "Bronze: {} hours, {} minutes",
            bronze_medal / 60,
            bronze_medal % 60
        );
    }
    println!();
}
