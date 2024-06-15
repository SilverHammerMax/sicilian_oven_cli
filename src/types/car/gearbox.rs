#[derive(Default, Clone, Copy)]
pub enum Gearbox {
    #[default]
    GearboxOne,
    GearboxTwo,
    GearboxThree,
    GearboxFour,
    GearboxFive,
}

impl Gearbox {
    pub fn brake_horsepower(&self) -> f64 {
        match self {
            Gearbox::GearboxOne => 0.0,
            Gearbox::GearboxTwo => 10.0,
            Gearbox::GearboxThree => 20.0,
            Gearbox::GearboxFour => 30.0,
            Gearbox::GearboxFive => 45.0,
        }
    }

    pub fn deterioration(&self) -> f64 {
        match self {
            Gearbox::GearboxOne => 0.01,
            Gearbox::GearboxTwo => 0.015,
            Gearbox::GearboxThree => 0.02,
            Gearbox::GearboxFour => 0.025,
            Gearbox::GearboxFive => 0.035,
        }
    }

    pub fn gearbox_type(&self) -> &str {
        match self {
            Gearbox::GearboxOne => "4S",
            Gearbox::GearboxTwo => "4S",
            Gearbox::GearboxThree => "5S",
            Gearbox::GearboxFour => "6S",
            Gearbox::GearboxFive => "6S",
        }
    }
}
