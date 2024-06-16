#[derive(Default, Clone, Copy)]
pub enum Gearbox {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Gearbox {
    pub fn brake_horsepower(&self) -> f64 {
        match self {
            Gearbox::One => 0.0,
            Gearbox::Two => 10.0,
            Gearbox::Three => 20.0,
            Gearbox::Four => 30.0,
            Gearbox::Five => 45.0,
        }
    }

    pub fn deterioration(&self) -> f64 {
        match self {
            Gearbox::One => 0.01,
            Gearbox::Two => 0.015,
            Gearbox::Three => 0.02,
            Gearbox::Four => 0.025,
            Gearbox::Five => 0.035,
        }
    }

    pub fn gearbox_type(&self) -> &str {
        match self {
            Gearbox::One => "4S",
            Gearbox::Two => "4S",
            Gearbox::Three => "5S",
            Gearbox::Four => "6S",
            Gearbox::Five => "6S",
        }
    }
}