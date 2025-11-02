use serde::{Deserialize, Serialize};

use crate::{
    runners::Runner,
    settings::InstalledGame
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub version: String,
}

impl Runner for Wine {
    fn run_game(&self, game: &InstalledGame) -> Result<(), String> {
        let settings = game.settings.upgrade().ok_or_else(|| "No reference to global settings".to_string())?;
        let settings = &settings.read().map_err(|e| format!("Error reading settings: {}", e))?;

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
