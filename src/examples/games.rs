use crate::examples::Example;
use crate::game::GameManifest;

pub struct Game {
    json: String,
}

impl Example for Game {
    type BuiltValue = GameManifest;

    fn json(&self) -> &str {
        &self.json
    }
}

impl Game {
    pub fn min() -> Self {
        Self {
            json: include_str!("../../examples/games/min.json").to_string(),
        }
    }

    pub fn simple_battle() -> Self {
        Self {
            json: include_str!("../../examples/games/simple_battle/simple_battle.json").to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::examples;
    use crate::examples::games::Game;

    #[test]
    fn it_serializes_min() {
        examples::run_example_round_trip_test(Game::min);
    }

    #[test]
    fn it_serializes_simple_battle() {
        examples::run_example_round_trip_test(Game::simple_battle);
    }
}
