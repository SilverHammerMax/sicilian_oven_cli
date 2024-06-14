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
        _ => panic!("Fix Added Car!"),
    }
}

pub fn choose_challenge() -> Challenge {
    let mut challenge_names = Vec::new();
    for challenge in challenge::CHALLENGES {
        challenge_names.push(format!(
            "{} {}",
            challenge.get_name(),
            challenge.get_medal()
        ));
    }
    let selection = dialoguer::Select::new()
        .with_prompt("Please Select a Challenge")
        .items(&challenge_names)
        .interact()
        .expect("Prompt Failed");

    challenge::CHALLENGES[selection].clone()
}

pub fn choose_major_city(region: Option<&city::Region>) -> &'static str {
    let mut major_cities = Vec::new();
    for code in constants::MAJOR_CITIES.iter() {
        major_cities.push(cities::CITIES.get(code).expect("Invalid City Code"));
    }

    major_cities.retain(|x| Some(x.get_region()) == region || region == None);
    let major_names: Vec<String> = major_cities.iter().map(|x| x.get_name().to_owned()).collect();

    let selection = dialoguer::Select::new()
        .with_prompt("What major city would you like to start in?")
        .items(&major_names)
        .interact()
        .expect("Prompt Failed");

    constants::MAJOR_CITIES[selection]
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

    let start_city = challenge.get_start_city();
    let end_city = challenge.get_end_city();

    match start_city {
        challenge::Location::City(code) => println!(
            "You will start in {}.",
            cities::CITIES
                .get(code)
                .expect("Invalid City Code")
                .get_name()
        ),
        challenge::Location::Region(region) => println!("You may start in any major city in {}.", region.get_name()),
        challenge::Location::Any => println!("You can start in whatever major city you prefer."),
    }

    match end_city {
        challenge::Location::City(code) => println!(
            "You will end in {}.",
            cities::CITIES
                .get(code)
                .expect("Invalid City Code")
                .get_name()
        ),
        challenge::Location::Region(region) => println!("You may start in any major city in {}.", region.get_name()),
        challenge::Location::Any => println!("You can start in whatever major city you prefer."),
    }

    println!();

    match challenge.get_car() {
        Some(car) => println!("You are using the {}.", car.get_car_type().get_name()),
        None => println!("You can use whatever car you prefer."),
    }

    println!();
    if challenge.get_name() != "Free Play" {
        let author_medal = challenge.get_medal_cutoff(0);
        let gold_medal = challenge.get_medal_cutoff(1);
        let silver_medal = challenge.get_medal_cutoff(2);
        let bronze_medal = challenge.get_medal_cutoff(3);

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
