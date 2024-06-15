use crate::types::*;

#[derive(Clone)]
pub struct Car {
    name: String,
    tires: car::tire::Tire,
    engine: car::engine::Engine,
    gearbox: car::gearbox::Gearbox,
    chassis: car::chassis::Chassis,
    fuel: f64,
    reliability: f64,
}

pub struct CarBuilder {
    name: Option<String>,
    tires: Option<car::tire::Tire>,
    engine: Option<car::engine::Engine>,
    gearbox: Option<car::gearbox::Gearbox>,
    chassis: Option<car::chassis::Chassis>,
}

impl CarBuilder {
    pub fn new() -> CarBuilder {
        CarBuilder {
            name: None,
            tires: None,
            engine: None,
            gearbox: None,
            chassis: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut CarBuilder {
        self.name = Some(name);
        self
    }

    pub fn tires(&mut self, tires: car::tire::Tire) -> &mut CarBuilder {
        self.tires = Some(tires);
        self
    }

    pub fn engine(&mut self, engine: car::engine::Engine) -> &mut CarBuilder {
        self.engine = Some(engine);
        self
    }

    pub fn gearbox(&mut self, gearbox: car::gearbox::Gearbox) -> &mut CarBuilder {
        self.gearbox = Some(gearbox);
        self
    }

    pub fn chassis(&mut self, chassis: car::chassis::Chassis) -> &mut CarBuilder {
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
            fuel: 0.0,
            reliability: 1.0,
        }
    }
}

impl Car {
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
        base_speed * multiplier
    }

    pub fn get_fuel(&self) -> f64 {
        self.fuel
    }

    pub fn refuel(&mut self) {
        self.fuel = self.chassis.tank_size();
    }

    pub fn calculate_travel_time(&self, road: &city::RoadTypes, distance: i32) -> f64 {
        distance as f64 / self.calculate_speed(road)
    }

    pub fn travel(&mut self) {
        self.fuel -= self.engine.fuel_usage();
    }
}