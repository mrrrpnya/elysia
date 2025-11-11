mod proton;
mod wine;

use serde::{Deserialize, Serialize};

pub use crate::runners::{proton::Proton, wine::Wine};
use crate::settings::InstalledGame;

pub trait Runner {
    fn run_game(&self, game: &InstalledGame) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Runners {
    Native,
    Wine(Wine),
    Proton(Proton),
}

impl Runner for Runners {
    fn run_game(&self, game: &InstalledGame) -> Result<(), String> {
        match self {
            Runners::Native => Ok(()),
            Runners::Wine(wine) => wine.run_game(game),
            Runners::Proton(proton) => proton.run_game(game),
        }
    }
}
