#[derive(Default, Clone, Copy)]
pub enum Chassis {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
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
