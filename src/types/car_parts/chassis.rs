use std::fmt::{Display, Formatter};
use strum;

#[derive(Default, Clone, Copy, strum::EnumIter)]
pub enum Chassis {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for Chassis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chassis::One => write!(f, "Stellare Ciabatta ({} kg, {} L)", self.weight(), self.tank_size()),
            Chassis::Two => write!(f, "Veloce Panettone ({} kg, {} L)", self.weight(), self.tank_size()),
            Chassis::Three => write!(f, "Ardente Rosetta ({} kg, {} L)", self.weight(), self.tank_size()),
            Chassis::Four => write!(f, "Solare Focaccia ({} kg, {} L)", self.weight(), self.tank_size()),
            Chassis::Five => write!(f, "Fiorente Filone ({} kg, {} L)", self.weight(), self.tank_size()),
        }
    }
}

impl Chassis {
    pub fn weight(&self) -> f64 {
        match self {
            Chassis::One => 705.0,
            Chassis::Two => 778.0,
            Chassis::Three => 861.0,
            Chassis::Four => 914.0,
            Chassis::Five => 996.0,
        }
    }

    pub fn tank_size(&self) -> f64 {
        match self {
            Chassis::One => 33.6,
            Chassis::Two => 37.0,
            Chassis::Three => 41.0,
            Chassis::Four => 43.5,
            Chassis::Five => 47.4,
        }
    }
}
