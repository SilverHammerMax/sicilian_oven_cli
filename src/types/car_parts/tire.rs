use std::fmt::{Display, Formatter};
use strum;

#[derive(Default, Clone, Copy, strum::EnumIter)]
pub enum Tire {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Display for Tire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tire::One => write!(f, "Stellare Volpe ({} AGC, {} GGC)", self.agc(), self.ggc()),
            Tire::Two => write!(f, "Veloce Orso ({} AGC, {} GGC)", self.agc(), self.ggc()),
            Tire::Three => write!(f, "Ardente Lupo ({} AGC, {} GGC)", self.agc(), self.ggc()),
            Tire::Four => write!(f, "Solare Cavallo ({} AGC, {} GGC)", self.agc(), self.ggc()),
            Tire::Five => write!(f, "Fiorente Roadrunner ({} AGC, {} GGC)", self.agc(), self.ggc()),
        }
    }
}

impl Tire {
    pub fn agc(&self) -> f64 {
        match self {
            Tire::One => 1.0,
            Tire::Two => 4.0,
            Tire::Three => 6.0,
            Tire::Four => 8.0,
            Tire::Five => 10.0,
        }
    }

    pub fn ggc(&self) -> f64 {
        match self {
            Tire::One => 10.0,
            Tire::Two => 8.0,
            Tire::Three => 6.0,
            Tire::Four => 4.0,
            Tire::Five => 1.0,
        }
    }
}
