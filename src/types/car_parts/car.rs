use crate::types::*;

#[derive(Clone, Copy)]
pub struct Car {
    name: &'static str,
    tires: car_parts::tire::Tire,
    engine: car_parts::engine::Engine,
    gearbox: car_parts::gearbox::Gearbox,
    chassis: car_parts::chassis::Chassis,
    fuel: f64,
    reliability: f64,
}

impl Car {
    pub const fn new(
        name: &'static str,
        tires: car_parts::tire::Tire,
        engine: car_parts::engine::Engine,
        gearbox: car_parts::gearbox::Gearbox,
        chassis: car_parts::chassis::Chassis,
    ) -> Car {
        Car {
            name,
            tires,
            engine,
            gearbox,
            chassis,
            fuel: 0.0,
            reliability: 1.0,
        }
    }

    pub fn name(&self) -> &str {
        self.name
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

    pub fn get_fuel(&self) -> f64 {
        self.fuel
    }

    pub fn refuel(&mut self, mut time: f64) {
        self.fuel = self.chassis.tank_size();
        time += 10.0 + 1.5 * (self.chassis().tank_size() - self.get_fuel());
    }

    pub fn repair(&mut self, mut time: f64) {
        self.reliability = 1.0;
        time += 145.0 - self.reliability * 100.0;
    }

    pub fn calculate_travel_time(&self, road: &city::RoadTypes, distance: i32) -> f64 {
        distance as f64 / self.calculate_speed(road)
    }

    pub fn travel(&mut self) {
        self.fuel -= self.engine.fuel_usage();
        self.reliability -= self.gearbox().deterioration();
    }
}
