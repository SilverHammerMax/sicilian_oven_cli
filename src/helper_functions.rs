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

pub fn choose_challenge(challenges: Res<challenge::ChallengesResource>, mut commands: Commands, mut next_state: ResMut<NextState<GameStates>>) {
    let selection = dialoguer::Select::new()
        .with_prompt("Please Select a Challenge")
        .items(challenges.challenges.as_slice())
        .interact()
        .expect("Prompt Failed");
    commands.insert_resource(challenges.challenges[selection].clone());
    next_state.set(GameStates::SetupChallenge);
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

pub fn setup_challenge(mut commands: Commands, mut next_state: ResMut<NextState<GameStates>>, cities: Res<cities::CityGraph>, mut cars: ResMut<car_parts::car::CarsResource>, challenge: Res<challenge::Challenge>) {
    println!(
        "\nWelcome to {}! {}.\n",
        challenge.name(),
        challenge.description()
    );
    println!("In this challenge, you will attempt to reach the cities of:\n");
    for city in challenge.cities() {
        println!("- {}", cities.get(city).expect("Invalid City Name"));
    }
    println!();

    let start_city = challenge.start_city();
    let end_city = challenge.end_city();

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

    if !dialoguer::Confirm::new()
        .with_prompt("Do you accept this challenge?")
        .interact()
        .expect("Prompt Failed")
    {
        next_state.set(GameStates::MainMenu);
    } else {
        let car = challenge
            .car()
            .unwrap_or_else(|| choose_car(cars.cars.as_mut_slice()));
        let missing_cities = challenge.cities().to_vec();
        let city_name = match challenge.start_city() {
            challenge::Location::City(name) => name.to_string(),
            challenge::Location::Region(region) => {
                choose_major_city(Some(region), &cities)
            }
            challenge::Location::Any => choose_major_city(None, &cities),
        };
        commands.insert_resource(ChallengeTime(0.0));
        commands.insert_resource(ChallengePath(vec![]));
        commands.insert_resource(MissingCities(missing_cities));
        commands.insert_resource(CurrentCity(city_name));
        commands.insert_resource(car);
        next_state.set(GameStates::RunChallenge);
    }
}

pub fn test_city_connections(city_graph: Res<cities::CityGraph>) {
    let mut cities = vec![];
    for city in city_graph.cities() {
        cities.push(city);
    }
    loop {
        let selection = dialoguer::Select::new()
            .with_prompt("Please Select a City")
            .items(&cities)
            .interact()
            .expect("Prompt Failed");
        for connection in city_graph.get_neighbors(cities[selection].name()) {
            println!("{:?}", connection);
        }
    }
}
