use crate::types::*;
use crate::*;

pub fn choose_car(cars: &mut Vec<car_parts::car::Car>) -> car_parts::car::Car {
    let car_names = cars.iter().map(|car| car.name()).collect::<Vec<_>>();
    let selection = dialoguer::Select::new()
        .with_prompt("Pick your car")
        .items(&car_names)
        .interact()
        .expect("Prompt Failed");

    cars[selection].clone()
}

pub fn choose_challenge() -> challenge::Challenge {
    let mut challenge_names = Vec::new();
    for challenge in challenge::initialize_challenges() {
        challenge_names.push(format!("{} {}", challenge, challenge.get_medal()));
    }
    let selection = dialoguer::Select::new()
        .with_prompt("Please Select a Challenge")
        .items(&challenge_names)
        .interact()
        .expect("Prompt Failed");

    challenge::initialize_challenges()[selection].clone()
}

pub fn choose_major_city(region: Option<&city::Region>) -> &'static str {
    let major_cities: Vec<String> = city::major_cities(region)
        .iter()
        .map(|code| {
            format!(
                "{} ({})",
                cities::CITIES
                    .get(code)
                    .expect("Invalid City Code")
                    .get_name(),
                cities::CITIES
                    .get(code)
                    .expect("Invalid City Code")
                    .get_region()
            )
        })
        .collect();

    let selection = dialoguer::Select::new()
        .with_prompt("What major city would you like to start in?")
        .items(&major_cities)
        .interact()
        .expect("Prompt Failed");

    city::major_cities(region)[selection]
}

pub fn selection_prompt(mut cars: &mut Vec<car_parts::car::Car>) -> challenge::Challenge {
    let mut challenge = None;
    while challenge.is_none() {
        let options = vec!["Challenges", "Random Cities", "Build Car"];
        let selection = dialoguer::Select::new()
            .with_prompt("What would you like to play?")
            .items(&options)
            .interact()
            .expect("Prompt Failed");
        match selection {
            0 => challenge = Some(choose_challenge()),
            1 => challenge = Some(challenge::random_challenge()),
            2 => cars.push(car_parts::car::car_build_prompt()),
            _ => panic!("Fix New Options!"),
        }
    }
    challenge.unwrap()
}

pub fn challenge_prompt(challenge: &challenge::Challenge) {
    println!(
        "\nWelcome to {}! In this challenge, you will attempt to reach the cities of:\n",
        challenge.name()
    );
    for city in challenge.cities() {
        println!(
            "- {} ({})",
            cities::CITIES
                .get(city)
                .expect("Invalid City Code")
                .get_name(),
            cities::CITIES
                .get(city)
                .expect("Invalid City Code")
                .get_region()
        );
    }
    println!();

    let start_city = challenge.start_city();
    let end_city = challenge.get_end_city();

    match start_city {
        challenge::Location::City(code) => println!(
            "You will start in {}.",
            cities::CITIES
                .get(code)
                .expect("Invalid City Code")
                .get_name()
        ),
        challenge::Location::Region(region) => {
            println!("You may start in any major city in {}.", region)
        }
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
        challenge::Location::Region(region) => {
            println!("You may start in any major city in {}.", region)
        }
        challenge::Location::Any => println!("You can start in whatever major city you prefer."),
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
    if challenge.medal_cutoffs().is_some() {
        let author_medal = challenge.medal_cutoffs().unwrap()[0];
        let gold_medal = challenge.medal_cutoffs().unwrap()[1];
        let silver_medal = challenge.medal_cutoffs().unwrap()[2];
        let bronze_medal = challenge.medal_cutoffs().unwrap()[3];

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
