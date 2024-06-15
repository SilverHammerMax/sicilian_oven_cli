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