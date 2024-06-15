pub struct Car {
    name: String,
    tires: super::tire::Tire,
    engine: super::engine::Engine,
    gearbox: super::gearbox::Gearbox,
    chassis: super::chassis::Chassis,
    fuel: f64,
}

pub struct CarBuilder {
    name: Option<String>,
    tires: Option<super::tire::Tire>,
    engine: Option<super::engine::Engine>,
    gearbox: Option<super::gearbox::Gearbox>,
    chassis: Option<super::chassis::Chassis>,
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

    pub fn tires(&mut self, tires: super::tire::Tire) -> &mut CarBuilder {
        self.tires = Some(tires);
        self
    }

    pub fn engine(&mut self, engine: super::engine::Engine) -> &mut CarBuilder {
        self.engine = Some(engine);
        self
    }

    pub fn gearbox(&mut self, gearbox: super::gearbox::Gearbox) -> &mut CarBuilder {
        self.gearbox = Some(gearbox);
        self
    }

    pub fn chassis(&mut self, chassis: super::chassis::Chassis) -> &mut CarBuilder {
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
        }
    }
}

impl Car {
    pub fn brake_horsepower(&self) -> f64 {
        self.gearbox.brake_horsepower() + self.engine.brake_horsepower()
    }
}