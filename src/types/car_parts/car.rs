use crate::types::*;
use crate::GameStates;
use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{BufReader, Write};
use strum::IntoEnumIterator;

#[derive(Resource)]
pub struct CarsResource {
    pub(crate) cars: Vec<Car>,
}

#[derive(Resource, Clone, serde::Serialize, serde::Deserialize)]
pub struct Car {
    name: String,
    tires: car_parts::tire::Tire,
    engine: car_parts::engine::Engine,
    gearbox: car_parts::gearbox::Gearbox,
    chassis: car_parts::chassis::Chassis,
    #[serde(skip)]
    fuel: f64,
    #[serde(skip)]
    reliability: f64,
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Car {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tires(&self) -> &car_parts::tire::Tire {
        &self.tires
    }

    pub fn engine(&self) -> &car_parts::engine::Engine {
        &self.engine
    }

    pub fn gearbox(&self) -> &car_parts::gearbox::Gearbox {
        &self.gearbox
    }

    pub fn chassis(&self) -> &car_parts::chassis::Chassis {
        &self.chassis
    }

    pub fn brake_horsepower(&self) -> f64 {
        self.gearbox.brake_horsepower() + self.engine.brake_horsepower()
    }

    pub fn weight(&self) -> f64 {
        self.engine.weight() + self.chassis.weight()
    }

    pub fn reliability(&self) -> f64 {
        self.reliability
    }

    pub fn calculate_speed(&self, road: &city::RoadTypes) -> f64 {
        let base_speed = 100.0 * self.reliability() * self.brake_horsepower() / self.weight();
        let multiplier = match road {
            city::RoadTypes::Highway => 2.0 * self.tires().agc(),
            city::RoadTypes::Asphalt => self.tires().agc(),
            city::RoadTypes::Cobblestone => 3.0 / 4.0 * self.tires().ggc(),
            city::RoadTypes::Unpaved => 0.5 * self.tires().ggc(),
            city::RoadTypes::Ferry => 0.0,
        };
        base_speed * multiplier / 60.0
    }

    pub fn fuel(&self) -> f64 {
        self.fuel
    }

    pub fn refuel(&mut self, time: &mut f64) {
        *time += 10.0 + 1.5 * (self.chassis().tank_size() - self.fuel());
        self.fuel = self.chassis().tank_size();
    }

    pub fn repair(&mut self, time: &mut f64) {
        *time += 145.0 - self.reliability() * 100.0;
        self.reliability = 1.0;
    }

    pub fn calculate_travel_time(&self, road: &city::RoadTypes, distance: i32) -> f64 {
        match road {
            city::RoadTypes::Ferry => 15.0 + 0.4 * distance as f64,
            _ => distance as f64 / self.calculate_speed(road),
        }
    }

    pub fn travel(&mut self, road: &city::RoadTypes) {
        match road {
            &city::RoadTypes::Ferry => (),
            _ => {
                self.fuel -= self.engine().fuel_usage();
                self.reliability -= self.gearbox().deterioration();
            }
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all("cars")?;
        let mut file = fs::File::create(format!(
            "cars/{}.json",
            self.name().to_lowercase().replace(" ", "_")
        ))?;
        file.write_all(serde_json::to_string(&self)?.into_bytes().as_slice())?;
        Ok(())
    }

    pub fn initialize() -> Vec<Car> {
        let mut cars = vec![
            CarBuilder::new()
                .name("Il Commandante")
                .tires(car_parts::tire::Tire::Four)
                .engine(car_parts::engine::Engine::One)
                .gearbox(car_parts::gearbox::Gearbox::Three)
                .chassis(car_parts::chassis::Chassis::Three)
                .build(),
            CarBuilder::new()
                .name("Il Grande")
                .tires(car_parts::tire::Tire::Three)
                .engine(car_parts::engine::Engine::Five)
                .gearbox(car_parts::gearbox::Gearbox::Four)
                .chassis(car_parts::chassis::Chassis::Five)
                .build(),
            CarBuilder::new()
                .name("Il Capo")
                .tires(car_parts::tire::Tire::Two)
                .engine(car_parts::engine::Engine::Two)
                .gearbox(car_parts::gearbox::Gearbox::Two)
                .chassis(car_parts::chassis::Chassis::One)
                .build(),
            CarBuilder::new()
                .name("Il Generalissimo")
                .tires(car_parts::tire::Tire::Four)
                .engine(car_parts::engine::Engine::Three)
                .gearbox(car_parts::gearbox::Gearbox::One)
                .chassis(car_parts::chassis::Chassis::Four)
                .build(),
        ];

        let cars_directory = fs::read_dir("cars");
        if let Ok(directory) = cars_directory {
            for path in directory.flatten() {
                if let Ok(file) = fs::File::open(path.path()) {
                    let file_reader = BufReader::new(file);
                    match serde_json::from_reader::<BufReader<fs::File>, Car>(file_reader) {
                        Ok(mut car) => {
                            car.fuel = car.chassis().tank_size();
                            car.reliability = 1.0;
                            cars.push(car)
                        }
                        Err(e) => println!("Failed to Deserialize Car: {}", e),
                    }
                }
            }
        }

        cars
    }

    pub fn build_prompt(
        mut next_state: ResMut<NextState<crate::GameStates>>,
        mut cars: ResMut<CarsResource>,
    ) {
        let mut car = CarBuilder::new();
        let mut main_options = vec![
            "Name".to_string(),
            "Tires".to_string(),
            "Engine".to_string(),
            "Gearbox".to_string(),
            "Chassis".to_string(),
            "Build!".to_string(),
        ];
        loop {
            let selection = dialoguer::Select::new()
                .with_prompt("What would you like to modify?")
                .items(&main_options)
                .interact()
                .expect("Prompt Failed");

            match selection {
                0 => {
                    let name: String = dialoguer::Input::new()
                        .with_prompt("Enter the Car's Name")
                        .with_initial_text("New Car")
                        .validate_with(|name: &String| match name.as_str() {
                            "" => Err("Name Cannot Be Empty"),
                            _ => Ok(()),
                        })
                        .interact_text()
                        .expect("Prompt Failed");
                    main_options[0] = format!("Name ({})", name);
                    car = car.name(name);
                }
                1 => {
                    let options: Vec<car_parts::tire::Tire> =
                        car_parts::tire::Tire::iter().collect();
                    let selection = dialoguer::Select::new()
                        .with_prompt("Please Select your Tires")
                        .items(&options)
                        .interact()
                        .expect("Prompt Failed");
                    main_options[1] = format!("Tires ({})", options[selection]);
                    car = car.tires(options[selection]);
                }
                2 => {
                    let options: Vec<car_parts::engine::Engine> =
                        car_parts::engine::Engine::iter().collect();
                    let selection = dialoguer::Select::new()
                        .with_prompt("Please Select your Engine")
                        .items(&options)
                        .interact()
                        .expect("Prompt Failed");
                    main_options[2] = format!("Engine ({})", options[selection]);
                    car = car.engine(options[selection]);
                }
                3 => {
                    let options: Vec<car_parts::gearbox::Gearbox> =
                        car_parts::gearbox::Gearbox::iter().collect();
                    let selection = dialoguer::Select::new()
                        .with_prompt("Please Select your Gearbox")
                        .items(&options)
                        .interact()
                        .expect("Prompt Failed");
                    main_options[3] = format!("Gearbox ({})", options[selection]);
                    car = car.gearbox(options[selection]);
                }
                4 => {
                    let options: Vec<car_parts::chassis::Chassis> =
                        car_parts::chassis::Chassis::iter().collect();
                    let selection = dialoguer::Select::new()
                        .with_prompt("Please Select your Chassis")
                        .items(&options)
                        .interact()
                        .expect("Prompt Failed");
                    main_options[4] = format!("Chassis ({})", options[selection]);
                    car = car.chassis(options[selection]);
                }
                5 => {
                    let built_car = car.build();
                    let save_status = Car::save(&built_car);
                    match save_status {
                        Ok(_) => (),
                        Err(e) => println!("Failed to Save Car: {}", e),
                    }
                    cars.cars.push(built_car);
                    next_state.set(crate::GameStates::MainMenu);
                    break;
                }
                _ => panic!("Not Yet Implemented!"),
            }
        }
    }

    pub fn deletion_prompt(
        mut cars: ResMut<CarsResource>,
        mut next_state: ResMut<NextState<GameStates>>,
    ) {
        loop {
            if cars.cars.len() == 4 {
                println!("No cars left to delete!");
                break;
            }
            let choice = dialoguer::Select::new()
                .items(&cars.cars[4..])
                .with_prompt("Which car would you like to delete")
                .interact()
                .expect("Prompt Failed") + 4;
            let confirmation = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "Are you sure you would like to delete {}?",
                    cars.cars[choice]
                ))
                .interact()
                .expect("Prompt Failed");
            if confirmation {
                let file_deletion = fs::remove_file(format!("cars/{}.json", cars.cars[choice].name.to_lowercase().replace(' ', "_")));
                match file_deletion {
                    Ok(_) => (),
                    Err(e) => println!("Failed to delete file: {}", e)
                }
                cars.cars.remove(choice);
                break;
            }
        }
        next_state.set(GameStates::MainMenu);
    }
}

pub struct CarBuilder {
    name: Option<String>,
    tires: Option<car_parts::tire::Tire>,
    engine: Option<car_parts::engine::Engine>,
    gearbox: Option<car_parts::gearbox::Gearbox>,
    chassis: Option<car_parts::chassis::Chassis>,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            tires: None,
            engine: None,
            gearbox: None,
            chassis: None,
        }
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn tires(mut self, tires: car_parts::tire::Tire) -> Self {
        self.tires = Some(tires);
        self
    }

    pub fn engine(mut self, engine: car_parts::engine::Engine) -> Self {
        self.engine = Some(engine);
        self
    }

    pub fn gearbox(mut self, gearbox: car_parts::gearbox::Gearbox) -> Self {
        self.gearbox = Some(gearbox);
        self
    }

    pub fn chassis(mut self, chassis: car_parts::chassis::Chassis) -> Self {
        self.chassis = Some(chassis);
        self
    }

    pub fn build(self) -> Car {
        Car {
            name: self.name.unwrap_or("New Car".to_string()),
            tires: self.tires.unwrap_or_default(),
            engine: self.engine.unwrap_or_default(),
            gearbox: self.gearbox.unwrap_or_default(),
            chassis: self.chassis.unwrap_or_default(),
            fuel: self.chassis.unwrap_or_default().tank_size(),
            reliability: 1.0,
        }
    }
}
