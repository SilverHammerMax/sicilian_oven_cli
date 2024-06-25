#![deny(clippy::unwrap_used)]
#![allow(clippy::match_overlapping_arm)]

use crate::types::*;

mod cities;
mod helper_functions;
mod types;

fn main() {
    let mut cars = car_parts::car::Car::initialize();
    let cities = cities::create_cities();
    loop {
        println!("Welcome to the game!");
        let mut challenge = helper_functions::selection_prompt(&mut cars, &cities);
        challenge_engine(&mut challenge, &mut cars, &cities);
    }
}

fn challenge_engine(
    challenge: &mut challenge::Challenge,
    cars: &mut Vec<car_parts::car::Car>,
    cities: &cities::CityGraph,
) {
    helper_functions::challenge_prompt(cities, challenge);
    match dialoguer::Confirm::new()
        .with_prompt("Do you accept this challenge?")
        .interact()
        .expect("Prompt Failed")
    {
        false => return,
        true => (),
    }
    let mut car = challenge
        .car()
        .unwrap_or_else(|| helper_functions::choose_car(cars));
    let mut missing_cities = challenge.cities().to_vec();
    let mut city_code = match challenge.start_city() {
        challenge::Location::City(code) => code.to_string(),
        challenge::Location::Region(region) => helper_functions::choose_major_city(Some(region)),
        challenge::Location::Any => helper_functions::choose_major_city(None),
    };
    let mut path = vec![];
    let mut time = 0.0;
    loop {
        let city_reference = cities.get(&city_code).expect("Invalid City Code");
        path.push(city_code.clone());
        missing_cities.retain(|code| *code != city_code);

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
        println!("Your reliability is {:.1}%.", (car.reliability() * 100.0));
        println!();
        println!(
            "Your path has been: {:?}",
            path.iter()
                .map(|code| cities.get(code).expect("Invalid City Code").get_name())
                .collect::<Vec<_>>()
        );
        println!();

        if !path.is_empty() {
            println!(
                "Your current list of missing cities is: {:?}",
                missing_cities
                    .iter()
                    .map(|code| cities.get(code).expect("Invalid City Code").get_name())
                    .collect::<Vec<_>>()
            );
        } else {
            println!("Your challenge is complete!");
        }

        println!();

        if car.fuel() - 3.0 * car.engine().fuel_usage() <= 0.0 {
            println!("WARNING: LOW FUEL");
        }
        if car.reliability() < 0.2 {
            println!("WARNING: CAR NEARLY DETERIORATED");
        }
        println!();
        let mut options: Vec<String> = Vec::new();
        for (code, distance, _) in cities.get_neighbors(city_code.as_str()) {
            let option = format!(
                "Go to {}, {} km",
                cities.get(&code).expect("Invalid City Code"),
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

        let next_city;
        if selection < cities.get_neighbors(city_code.as_str()).len() {
            next_city = cities
                .get_neighbors(&city_code)
                .get(selection)
                .expect("Out of Range")
                .clone();
            car.travel(&next_city.2);
            time += car.calculate_travel_time(&next_city.2, next_city.1);
            city_code.clone_from(&next_city.0);
        } else if selection == cities.get_neighbors(&city_code).len() {
            break;
        } else if city_reference.is_major()
            && selection == cities.get_neighbors(&city_code).len() + 1
        {
            car.refuel(&mut time);
            path.pop();
        } else if city_reference.is_major()
            && selection == cities.get_neighbors(&city_code).len() + 2
        {
            car.repair(&mut time);
            path.pop();
        }
        if car.fuel() <= 0.0 {
            println!("Ran out of fuel! Sorry, game over :(");
            break;
        }
        if car.reliability() <= 0.0 {
            println!("Car too deteriorated! Sorry, game over :(");
            break;
        }
    }

    println!();
    if missing_cities.is_empty()
        && (&challenge::Location::City(city_code.clone()) == challenge.get_end_city()
            || &challenge::Location::Region(
                cities
                    .get(&city_code)
                    .expect("Invalid City Code")
                    .get_region()
                    .clone(),
            ) == challenge.get_end_city()
            || challenge.get_end_city() == &challenge::Location::Any)
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

            match challenge.get_medal() {
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
