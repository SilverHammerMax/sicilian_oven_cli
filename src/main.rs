#![deny(clippy::unwrap_used)]
#![allow(clippy::match_overlapping_arm)]

use crate::types::*;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

mod cities;
mod helper_functions;
mod types;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameStates {
    #[default]
    MainMenu,
    CarBuilding,
    ConnectionTester,
    ChooseChallenge,
    RunChallenge,
    RandomChallenge,
}

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<GameStates>()
        .add_systems(PreStartup, setup)
        .add_systems(OnEnter(GameStates::MainMenu), menu)
        .add_systems(
            OnEnter(GameStates::CarBuilding),
            car_parts::car::Car::build_prompt,
        )
        .add_systems(OnEnter(GameStates::ConnectionTester), helper_functions::test_city_connections)
        .add_systems(OnEnter(GameStates::ChooseChallenge), helper_functions::choose_challenge)
        .add_systems(OnEnter(GameStates::RunChallenge), challenge_engine)
        .add_systems(OnEnter(GameStates::RandomChallenge), challenge::Challenge::random)
        .run();
}

fn setup(mut commands: Commands) {
    let cities = cities::create_cities();
    let cars = car_parts::car::CarsResource {
        cars: car_parts::car::Car::initialize(),
    };
    let challenges = challenge::ChallengesResource {
        challenges: challenge::Challenge::initialize(),
    };
    commands.insert_resource(cities);
    commands.insert_resource(cars);
    commands.insert_resource(challenges);
}

fn menu(mut next_state: ResMut<NextState<GameStates>>) {
    println!("Welcome to the game!");
    let selection = dialoguer::Select::new()
        .with_prompt("What would you like to do?")
        .items(&[
            "Challenges",
            "Random Cities",
            "Build a Car",
            "Test City Connections",
        ])
        .interact()
        .expect("Prompt Failed");

    match selection {
        0 => next_state.set(GameStates::ChooseChallenge),
        1 => next_state.set(GameStates::RandomChallenge),
        2 => next_state.set(GameStates::CarBuilding),
        3 => next_state.set(GameStates::ConnectionTester),
        _ => panic!("Fix New Options!"),
    }
}

fn challenge_engine(
    challenge: Res<challenge::Challenge>,
    mut cars: ResMut<car_parts::car::CarsResource>,
    cities: Res<cities::CityGraph>,
    mut next_state: ResMut<NextState<GameStates>>
) {
    let mut challenge = challenge.to_owned();
    helper_functions::challenge_prompt(&cities, &challenge);
    if !dialoguer::Confirm::new()
        .with_prompt("Do you accept this challenge?")
        .interact()
        .expect("Prompt Failed")
    {
        return;
    }
    let mut car = challenge
        .car()
        .unwrap_or_else(|| helper_functions::choose_car(cars.cars.as_mut_slice()));
    let mut missing_cities = challenge.cities().to_vec();
    let mut city_name = match challenge.start_city() {
        challenge::Location::City(name) => name.to_string(),
        challenge::Location::Region(region) => {
            helper_functions::choose_major_city(Some(region), &cities)
        }
        challenge::Location::Any => helper_functions::choose_major_city(None, &cities),
    };
    let mut path = vec![];
    let mut time = 0.0;
    loop {
        let city_reference = cities.get(&city_name).expect("Invalid City Name");
        path.push(city_name.clone());
        missing_cities.retain(|code| *code != city_name);

        println!();
        println!("Welcome to {}!", city_reference);
        println!();
        println!(
            "Your current time is {} hour(s) and {} minute(s)!",
            (time / 60.0) as i32,
            (time % 60.0) as i32
        );
        println!();
        println!("Your fuel is {:.1}L.", car.fuel());
        println!("Your reliability is {:.1}%.", car.reliability() * 100.0);
        println!();
        println!("Your path has been: {:?}", path);
        println!();

        if missing_cities.is_empty() {
            println!("Your challenge is complete!");
        } else {
            println!(
                "Your current list of missing cities is: {:?}",
                missing_cities
            );
        }

        println!();

        if car.fuel() <= 3.0 * car.engine().fuel_usage() {
            println!("WARNING: LOW FUEL");
        }
        if car.reliability() <= 3.0 * car.gearbox().deterioration() {
            println!("WARNING: CAR NEARLY DETERIORATED");
        }
        println!();
        let mut options: Vec<String> = Vec::new();
        let neighbors = cities.get_neighbors(city_name.as_str());
        for (name, distance, _) in &neighbors {
            let option = format!(
                "Go to {}, {} km",
                cities.get(name).expect("Invalid City Name"),
                distance
            );
            options.push(option);
        }
        options.push("Submit your challenge or return to main menu".to_string());
        if city_reference.is_major() {
            options.push("Refuel".to_string());
            options.push("Repair".to_string());
        }

        let selection = dialoguer::Select::new()
            .with_prompt("Where would you like to go?")
            .items(&options)
            .interact()
            .expect("Prompt Failed");

        if selection < neighbors.len() {
            let (next_city_name, distance, road) = neighbors.get(selection).expect("Out of Range");
            car.travel(road);
            time += car.calculate_travel_time(road, *distance);
            city_name.clone_from(next_city_name);
        } else if selection == neighbors.len() {
            break;
        } else if city_reference.is_major() && selection == neighbors.len() + 1 {
            car.refuel(&mut time);
            path.push("Refuel".to_string());
        } else if city_reference.is_major() && selection == neighbors.len() + 2 {
            car.repair(&mut time);
            path.push("Repair".to_string());
        }
        if car.fuel() <= 0.0 || car.reliability() <= 0.0 {
            break;
        }
    }

    if car.fuel() <= 0.0 {
        println!("Ran out of fuel! Sorry, game over :(");
        return;
    }

    if car.reliability() <= 0.0 {
        println!("Car too deteriorated! Sorry, game over :(");
        return;
    }

    println!();
    if missing_cities.is_empty()
        && (&challenge::Location::City(city_name.clone()) == challenge.end_city()
            || &challenge::Location::Region(
                cities
                    .get(&city_name)
                    .expect("Invalid City Code")
                    .region()
                    .clone(),
            ) == challenge.end_city()
            || challenge.end_city() == &challenge::Location::Any)
    {
        println!(
            "Congratulations! You've completed the {} challenge!",
            challenge
        );
        println!();
        println!(
            "You completed it in {} hour(s) and {} minute(s)!",
            (time / 60.0) as i32,
            (time % 60.0) as i32
        );
        println!();
        if let Some(cutoffs) = challenge.medal_cutoffs() {
            let author_cutoff = cutoffs[0] as f64;
            let gold_cutoff = cutoffs[1] as f64;
            let silver_cutoff = cutoffs[2] as f64;
            let bronze_cutoff = cutoffs[3] as f64;

            if time <= author_cutoff {
                challenge.set_medal(medal::Medal::Author);
            } else if time <= gold_cutoff {
                challenge.set_medal(medal::Medal::Gold);
            } else if time <= silver_cutoff {
                challenge.set_medal(medal::Medal::Silver);
            } else if time <= bronze_cutoff {
                challenge.set_medal(medal::Medal::Bronze);
            }

            match challenge.medal() {
                medal::Medal::Author => println!("This is an incredible time! You've won the author medal!"),
                medal::Medal::Gold => println!("This is an amazing time! You've won the gold medal!"),
                medal::Medal::Silver => println!("This is a great time! You've won the silver medal!"),
                medal::Medal::Bronze => println!("You're getting there! You've won the bronze medal!"),
                medal::Medal::None => println!("You're getting there, but that time unfortunately wasn't fast enough to win a medal. Please try again!")
            }
        }
    } else {
        println!("Sorry, you were unsuccessful. Better luck next time!");
    }
    next_state.set(GameStates::MainMenu)
}
