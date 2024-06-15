#[derive(Default)]
pub enum Engine {
    #[default]
    EngineOne,
    EngineTwo,
    EngineThree,
    EngineFour,
    EngineFive,
}

impl Engine {
    pub fn get_brake_horsepower(&self) -> f64 {
        match self {
            Engine::EngineOne => 112.0,
            Engine::EngineTwo => 200.0,
            Engine::EngineThree => 220.0,
            Engine::EngineFour => 276.0,
            Engine::EngineFive => 320.0,
        }
    }

    pub fn get_weight(&self) -> f64 {
        match self {
            Engine::EngineOne => 235.0,
            Engine::EngineTwo => 357.0,
            Engine::EngineThree => 372.0,
            Engine::EngineFour => 406.0,
            Engine::EngineFive => 432.0,
        }
    }
}