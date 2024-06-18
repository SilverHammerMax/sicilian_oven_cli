use crate::types::*;
use crate::types::city::RoadTypes;

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
    pub fn new(
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
            fuel: chassis.tank_size(),
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
            RoadTypes::Ferry => 15.0 + 2.5 * distance as f64,
            _ => distance as f64 / self.calculate_speed(road)
        }
    }

    pub fn travel(&mut self, road: &RoadTypes) {
        match road {
            &RoadTypes::Ferry => (),
            _ => {
                self.fuel -= self.engine.fuel_usage();
                self.reliability -= self.gearbox().deterioration();
            }
        }
    }
}

pub fn initialize_cars() -> Vec<Car> {
    vec![
        Car::new(
            "Il Commandante",
            car_parts::tire::Tire::Four,
            car_parts::engine::Engine::One,
            car_parts::gearbox::Gearbox::Three,
            car_parts::chassis::Chassis::Three,
        ),
        Car::new(
            "Il Grande",
            car_parts::tire::Tire::Three,
            car_parts::engine::Engine::Five,
            car_parts::gearbox::Gearbox::Four,
            car_parts::chassis::Chassis::Five,
        ),
        Car::new(
            "Il Capo",
            car_parts::tire::Tire::Two,
            car_parts::engine::Engine::Two,
            car_parts::gearbox::Gearbox::Two,
            car_parts::chassis::Chassis::One,
        ),
        Car::new(
            "Il Generalissimo",
            car_parts::tire::Tire::Four,
            car_parts::engine::Engine::Three,
            car_parts::gearbox::Gearbox::One,
            car_parts::chassis::Chassis::Two,
        ),
    ]
}
