use serde::{Deserialize, Serialize};

use crate::{
    runners::Runner,
    settings::{GlobalSettings, InstalledGame}
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub version: String,
}

impl Runner for Wine {
    fn run_game(&self, settings: &GlobalSettings, game: &InstalledGame) -> Result<(), String> {
        let components_path = &settings.components_directory.join("wine");
        let exe = components_path.join(&self.version).join("bin/wine");
        let prefix = &settings
            .wineprefixes_directory
            .join(&game.biz_name)
            .to_string_lossy()
            .into_owned();

        println!(
            "WINEPREFIX=\"{}\" {:?} {:?}",
            &prefix,
            &exe,
            &game.install_path.join(&game.executable_path)
        );

        Ok(())
    }
}
