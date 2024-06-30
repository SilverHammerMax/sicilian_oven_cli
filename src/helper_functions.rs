use crate::types::*;
use crate::*;

pub fn choose_car(cars: &mut [car_parts::car::Car]) -> car_parts::car::Car {
    let selection = dialoguer::Select::new()
        .with_prompt("Pick your car")
        .items(cars)
        .interact()
        .expect("Prompt Failed");

    cars[selection].clone()
}

pub fn choose_challenge(challenges: &mut [challenge::Challenge]) -> &mut challenge::Challenge {
    let selection = dialoguer::Select::new()
        .with_prompt("Please Select a Challenge")
        .items(challenges)
        .interact()
        .expect("Prompt Failed");

    &mut challenges[selection]
}

pub fn choose_major_city(region: Option<&city::Region>, cities: &cities::CityGraph) -> String {
    let major_cities: Vec<String> = city::major_cities(region, cities);

    let selection = dialoguer::Select::new()
        .with_prompt("What major city would you like to start in?")
        .items(&major_cities)
        .interact()
        .expect("Prompt Failed");

    major_cities[selection].clone()
}

pub fn challenge_prompt(cities: &cities::CityGraph, challenge: &challenge::Challenge) {

    println!(
        "\nWelcome to {}! {}.\n",
        challenge.name(),
        challenge.description()
    );
    println!(
        "In this challenge, you will attempt to reach the cities of:\n"
    );
    for city in challenge.cities() {
        println!("- {}", cities.get(city).expect("Invalid City Name"));
    }
    println!();

    let start_city = challenge.start_city();
    let end_city = challenge.get_end_city();

    match start_city {
        challenge::Location::City(city) => println!("You will start in {}.", city),
        challenge::Location::Region(region) => {
            println!("You may start in any major city in {}.", region)
        }
        challenge::Location::Any => println!("You can start in whatever major city you prefer."),
    }

    match end_city {
        challenge::Location::City(city) => println!("You will end in {}.", city),
        challenge::Location::Region(region) => {
            println!("You may end in any major city in {}.", region)
        }
        challenge::Location::Any => println!("You can end in whatever major city you prefer."),
    }

    println!();

    match challenge.car() {
        Some(car) => println!(
            "You are using the {} with a {}L {} engine and a {} gearbox.",
            car,
            car.engine().fuel_usage(),
            car.engine().engine_type(),
            car.gearbox().gearbox_type()
        ),
        None => println!("You can use whatever car you prefer."),
    }

    println!();
    if let Some(cutoffs) = challenge.medal_cutoffs() {
        let author_medal = cutoffs[0];
        let gold_medal = cutoffs[1];
        let silver_medal = cutoffs[2];
        let bronze_medal = cutoffs[3];

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
