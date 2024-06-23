use std::fmt::{Display, Formatter};
use strum;

#[derive(Default, Clone, Copy, strum::EnumIter)]
pub enum Gearbox {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for Gearbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gearbox::One => write!(f, "Stellare Provolone"),
            Gearbox::Two => write!(f, "Veloce Mozzarella"),
            Gearbox::Three => write!(f, "Ardente Gorgonzola"),
            Gearbox::Four => write!(f, "Solare Cheddar"),
            Gearbox::Five => write!(f, "Fiorente Parmesan"),
        }
    }
}

impl Gearbox {
    pub fn brake_horsepower(&self) -> f64 {
        match self {
            Gearbox::One => 0.0,
            Gearbox::Two => 40.0,
            Gearbox::Three => 80.0,
            Gearbox::Four => 120.0,
            Gearbox::Five => 220.0,
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
