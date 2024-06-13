#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::match_overlapping_arm)]

use crate::types::*;
use std::collections::HashMap;

mod cities;
mod constants;
mod helper_functions;
mod types;

fn main() {
    let mut medals: HashMap<challenge::Challenge, medal::Medal> = HashMap::from([
        (challenge::Challenge::RagusanRide, medal::Medal::None),
        (challenge::Challenge::BigCarBigCities, medal::Medal::None),
        (
            challenge::Challenge::ARideAroundMountEtna,
            medal::Medal::None,
        ),
        (challenge::Challenge::TheGodfather, medal::Medal::None),
        (challenge::Challenge::FreePlay, medal::Medal::None),
    ]);
    loop {
        println!("Welcome to the game!");
        let challenge = helper_functions::choose_challenge(&medals);
        challenge_engine(challenge, &mut medals);
    }
}

fn challenge_engine(
    challenge: challenge::Challenge,
    medals: &mut HashMap<challenge::Challenge, medal::Medal>,
) {
    helper_functions::challenge_prompt(&challenge);
    let selection = dialoguer::Confirm::new()
        .with_prompt("Do you accept this challenge?")
        .interact()
        .expect("Prompt Failed");
    if !selection {
        return;
    }
    let mut car = challenge
        .get_car()
        .unwrap_or_else(|| helper_functions::choose_car());
    let mut missing_cities = challenge.get_cities();
    let start_city = challenge
        .get_starting_city()
        .unwrap_or_else(|| helper_functions::choose_major_city());
    let mut city_code = start_city;
    let mut path = vec![];
    let mut time = 0.0;
    loop {
        let city_reference = cities::CITIES.get(city_code).expect("Invalid City Code");
        path.push(city_code);
        missing_cities.retain(|&x| x != city_code);

        println!();
        println!("Welcome to {}!", city_reference.get_name());
        println!();
        println!(
            "Your current time is {} hour(s) and {} minute(s)!",
            (time / 60.0) as i32,
            (time % 60.0) as i32
        );
        println!();
        println!("Your fuel is {}.", car.get_fuel_level() as i32);
        println!();
        println!("Your path has been: {:?}", path);
        println!();

        if path.len() > 0 {
            println!(
                "Your current list of missing cities is: {:?}",
                missing_cities
            );
        } else {
            println!("Your challenge is complete!");
        }

        println!();

        if car.get_fuel_level() - 3.0 * car.get_car_type().get_mileage() < 0.0 {
            println!("WARNING: LOW FUEL");
            println!();
        }
        let mut options: Vec<String> = Vec::new();
        for (code, distance, _) in city_reference.get_cities() {
            let option = format!(
                "Go to {}, {} km",
                cities::CITIES
                    .get(code)
                    .expect("Invalid City Code")
                    .get_name(),
                distance
            );
            options.push(option);
        }
        options.push("Submit your challenge or return to main menu".to_string());
        if city_reference.can_refuel() {
            options.push("Refuel".to_string());
        }

        let selection = dialoguer::Select::new()
            .with_prompt("Where would you like to go?")
            .items(&options)
            .interact()
            .expect("Prompt Failed");

        let next_city;
        if selection < city_reference.get_cities().len() {
            next_city = city_reference
                .get_cities()
                .get(selection)
                .expect("Out of Range");
            car.travel();
            time += next_city.1 as f64 * 60.0 * next_city.2.time_multiplier()
                / car.get_car_type().get_horsepower();
            city_code = next_city.0;
        } else if selection == city_reference.get_cities().len() {
            break;
        } else if city_reference.can_refuel() && selection == city_reference.get_cities().len() + 1
        {
            car.refuel();
            path.pop();
        }
        if car.get_fuel_level() <= 0.0 {
            println!("Ran out of fuel! Sorry, game over :(");
            break;
        }
    }

    println!();
    if challenge != challenge::Challenge::FreePlay {
        if missing_cities.is_empty()
            && (Some(city_code) == challenge.get_ending_city()
                || challenge.get_ending_city() == None)
        {
            println!(
                "Congratulations! You've completed the {} challenge!",
                challenge.get_name()
            );
            println!();
            println!(
                "You completed it in {} hour(s) and {} minute(s)!",
                (time / 60.0) as i32,
                (time % 60.0) as i32
            );
            println!();
            match challenge.get_medal(time) {
                medal::Medal::Author => println!("This is an incredible time! You've won the author medal!"),
                medal::Medal::Gold => println!("This is an amazing time! You've won the gold medal!"),
                medal::Medal::Silver => println!("This is a great time! You've won the silver medal!"),
                medal::Medal::Bronze => println!("You're getting there! You've won the bronze medal!"),
                medal::Medal::None => println!("You're getting there, but that time unfortunately wasn't fast enough to win a medal. Please try again!")
            }
            medals.insert(challenge.clone(), challenge.get_medal(time));
        } else {
            println!("Sorry, you were unsuccessful. Better luck next time!");
        }
    }
}
