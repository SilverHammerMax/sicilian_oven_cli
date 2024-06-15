#[derive(Default, Clone, Copy)]
pub enum Engine {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Engine {
    pub fn brake_horsepower(&self) -> f64 {
        match self {
            Engine::One => 112.0,
            Engine::Two => 200.0,
            Engine::Three => 220.0,
            Engine::Four => 276.0,
            Engine::Five => 320.0,
        }
    }

    pub fn weight(&self) -> f64 {
        match self {
            Engine::One => 235.0,
            Engine::Two => 357.0,
            Engine::Three => 372.0,
            Engine::Four => 406.0,
            Engine::Five => 432.0,
        }
    }

    pub fn fuel_usage(&self) -> f64 {
        match self {
            Engine::One => 2.4,
            Engine::Two => 3.2,
            Engine::Three => 3.8,
            Engine::Four => 4.6,
            Engine::Five => 5.2,
        }
    }

    pub fn engine_type(&self) -> &str {
        match self {
            Engine::One => "I6",
            Engine::Two => "V6",
            Engine::Three => "V8",
            Engine::Four => "V10",
            Engine::Five => "V12",
        }
    }
}
