use serde::{Deserialize, Serialize};

use crate::{runners::Runner, settings::InstalledGame};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proton {
    pub version: String,
}

impl Runner for Proton {
    fn run_game(&self, game: &InstalledGame) -> Result<(), String> {
        let settings = game.settings.upgrade().ok_or_else(|| "No reference to global settings".to_string())?;
        let settings = &settings.read().map_err(|e| format!("Error reading settings: {}", e))?;

        let components_path = settings.components_directory.join("proton");
        let proton_path = components_path.join(&self.version);

        let umu_dir = settings.components_directory.join("umu-launcher");
        let umu_run = umu_dir.join("umu-run");

        let prefix = settings
            .wineprefixes_directory
            .join(&game.biz_name)
            .to_string_lossy()
            .into_owned();

        println!(
            "PROTONPATH=\"{}\" PREFIX=\"{}\" {:?} {:?}",
            proton_path.display(),
            prefix,
            umu_run,
            game.install_path.join(&game.executable_path)
        );

        Ok(())
    }
}
