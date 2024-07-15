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
    SetupChallenge,
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
        .add_systems(OnEnter(GameStates::SetupChallenge), helper_functions::setup_challenge)
        .add_systems(Update, challenge_engine.run_if(in_state(GameStates::RunChallenge)))
        .add_systems(OnExit(GameStates::RunChallenge), challenge_finish)
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

#[derive(Resource)]
struct ChallengeTime(f64);
#[derive(Resource)]
struct ChallengePath(Vec<String>);
#[derive(Resource)]
struct MissingCities(Vec<String>);
#[derive(Resource)]
struct CurrentCity(String);

fn challenge_engine(
    mut car: ResMut<car_parts::car::Car>,
    mut time: ResMut<ChallengeTime>,
    mut path: ResMut<ChallengePath>,
    mut missing_cities: ResMut<MissingCities>,
    mut current_city: ResMut<CurrentCity>,
    cities: Res<cities::CityGraph>,
    mut next_state: ResMut<NextState<GameStates>>
) {
    let city_reference = cities.get(&current_city.0).expect("Invalid City Name");
    path.0.push(current_city.0.clone());
    missing_cities.0.retain(|city_name| *city_name != current_city.0);

    println!();
    println!("Welcome to {}!", city_reference);
    println!();
    println!(
        "Your current time is {} hour(s) and {} minute(s)!",
        (time.0 / 60.0) as i32,
        (time.0 % 60.0) as i32
    );
    println!();
    println!("Your fuel is {:.1}L.", car.fuel());
    println!("Your reliability is {:.1}%.", car.reliability() * 100.0);
    println!();
    println!("Your path has been: {:?}", path.0);
    println!();

    if missing_cities.0.is_empty() {
        println!("Your challenge is complete!");
    } else {
        println!(
            "Your current list of missing cities is: {:?}",
            missing_cities.0
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
    let neighbors = cities.get_neighbors(current_city.0.as_str());
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
        time.0 += car.calculate_travel_time(road, *distance);
        current_city.0.clone_from(next_city_name);
    } else if selection == neighbors.len() {
        next_state.set(GameStates::MainMenu)
    } else if city_reference.is_major() && selection == neighbors.len() + 1 {
        car.refuel(&mut time.0);
        path.0.push("Refuel".to_string());
    } else if city_reference.is_major() && selection == neighbors.len() + 2 {
        car.repair(&mut time.0);
        path.0.push("Repair".to_string());
    }
    if car.fuel() <= 0.0 || car.reliability() <= 0.0 {
        next_state.set(GameStates::MainMenu)
    }
}

fn challenge_finish(car: Res<car_parts::car::Car>, mut challenge: ResMut<challenge::Challenge>, cities: Res<cities::CityGraph>, time: Res<ChallengeTime>, missing_cities: Res<MissingCities>, current_city: Res<CurrentCity>) {
    if car.fuel() <= 0.0 {
        println!("Ran out of fuel! Sorry, game over :(");
        return;
    }

    if car.reliability() <= 0.0 {
        println!("Car too deteriorated! Sorry, game over :(");
        return;
    }

    println!();
    if missing_cities.0.is_empty()
        && (&challenge::Location::City(current_city.0.clone()) == challenge.end_city()
        || &challenge::Location::Region(
        cities
            .get(&current_city.0)
            .expect("Invalid City Code")
            .region()
            .clone(),
    ) == challenge.end_city()
        || challenge.end_city() == &challenge::Location::Any)
    {
        println!(
            "Congratulations! You've completed the {} challenge!",
            *challenge
        );
        println!();
        println!(
            "You completed it in {} hour(s) and {} minute(s)!",
            (time.0 / 60.0) as i32,
            (time.0 % 60.0) as i32
        );
        println!();
        if let Some(cutoffs) = challenge.medal_cutoffs() {
            let author_cutoff = cutoffs[0] as f64;
            let gold_cutoff = cutoffs[1] as f64;
            let silver_cutoff = cutoffs[2] as f64;
            let bronze_cutoff = cutoffs[3] as f64;

            if time.0 <= author_cutoff {
                challenge.set_medal(medal::Medal::Author);
            } else if time.0 <= gold_cutoff {
                challenge.set_medal(medal::Medal::Gold);
            } else if time.0 <= silver_cutoff {
                challenge.set_medal(medal::Medal::Silver);
            } else if time.0 <= bronze_cutoff {
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
}