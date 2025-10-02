mod proton;
mod wine;

pub use proton::Proton;
pub use wine::Wine;

use serde::{Deserialize, Serialize};

use crate::settings::InstalledGame;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Runner {
    Native,
    Wine(Wine),
    Proton(Proton),
}

impl Runner {
    pub fn run_game(&self, game: &InstalledGame) {
        match self {
            Runner::Native => {}
            Runner::Wine(wine) => {
                wine.run_game(game);
            }
            Runner::Proton(_) => {}
        }
    }
}
