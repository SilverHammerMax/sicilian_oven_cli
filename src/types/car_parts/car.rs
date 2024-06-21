use std::fmt::{Display, Formatter};
use crate::types::*;

#[derive(Clone)]
pub struct Car {
    name: String,
    tires: car_parts::tire::Tire,
    engine: car_parts::engine::Engine,
    gearbox: car_parts::gearbox::Gearbox,
    chassis: car_parts::chassis::Chassis,
    fuel: f64,
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
            city::RoadTypes::Highway => 2.0 * self.tires.agc(),
            city::RoadTypes::Asphalt => self.tires.agc(),
            city::RoadTypes::Cobblestone => 3.0 / 4.0 * self.tires.ggc(),
            city::RoadTypes::Unpaved => 0.5 * self.tires.ggc(),
            city::RoadTypes::Ferry => 0.0,
        };
        base_speed * multiplier / 60.0
    }

    pub fn fuel(&self) -> f64 {
        self.fuel
    }

    pub fn refuel(&mut self, time: &mut f64) {
        self.fuel = self.chassis.tank_size();
        *time += 10.0 + 1.5 * (self.chassis().tank_size() - self.fuel());
    }

    pub fn repair(&mut self, time: &mut f64) {
        self.reliability = 1.0;
        *time += 145.0 - self.reliability * 100.0;
    }

    pub fn calculate_travel_time(&self, road: &city::RoadTypes, distance: i32) -> f64 {
        match road {
            city::RoadTypes::Ferry => 15.0 + 2.5 * distance as f64,
            _ => distance as f64 / self.calculate_speed(road)
        }
    }

    pub fn travel(&mut self, road: &city::RoadTypes) {
        match road {
            &city::RoadTypes::Ferry => (),
            _ => {
                self.fuel -= self.engine.fuel_usage();
                self.reliability -= self.gearbox().deterioration();
            }
        }
    }

    pub fn initialize() -> Vec<Car> {
        vec![
            CarBuilder::new()
                .name("Il Comandante".to_string())
                .tires(car_parts::tire::Tire::Four)
                .engine(car_parts::engine::Engine::One)
                .gearbox(car_parts::gearbox::Gearbox::Three)
                .chassis(car_parts::chassis::Chassis::Three)
                .build(),
            CarBuilder::new()
                .name("Il Grande".to_string())
                .tires(car_parts::tire::Tire::Three)
                .engine(car_parts::engine::Engine::Five)
                .gearbox(car_parts::gearbox::Gearbox::Four)
                .chassis(car_parts::chassis::Chassis::Five)
                .build(),
            CarBuilder::new()
                .name("Il Capo".to_string())
                .tires(car_parts::tire::Tire::Two)
                .engine(car_parts::engine::Engine::Two)
                .gearbox(car_parts::gearbox::Gearbox::Two)
                .chassis(car_parts::chassis::Chassis::One)
                .build(),
            CarBuilder::new()
                .name("Il Generalissimo".to_string())
                .tires(car_parts::tire::Tire::Four)
                .engine(car_parts::engine::Engine::Three)
                .gearbox(car_parts::gearbox::Gearbox::One)
                .chassis(car_parts::chassis::Chassis::Four)
                .build(),
        ]
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

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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
            name: self.name.unwrap_or_default(),
            tires: self.tires.unwrap_or_default(),
            engine: self.engine.unwrap_or_default(),
            gearbox: self.gearbox.unwrap_or_default(),
            chassis: self.chassis.unwrap_or_default(),
            fuel: self.chassis.unwrap_or_default().tank_size(),
            reliability: 1.0,
        }
    }
}
