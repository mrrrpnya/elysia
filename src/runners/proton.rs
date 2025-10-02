use freya::prelude::{Readable, Signal};
use serde::{Deserialize, Serialize};

use crate::settings::{GlobalSettings, InstalledGame};
use crate::utils::umu::setup_umu;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proton {
    pub version: String,
}

impl Proton {
    pub fn run_game(&self, game: &InstalledGame) -> Result<(), String> {
        let ctx = &dioxus::hooks::use_context::<Signal<GlobalSettings>>();
        let settings = &ctx.read();

        let components_path = settings.components_directory.join("proton");
        let proton_path = components_path.join(&self.version);

        // todo: async?
        // let umu_run = setup_umu().await?;
        
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
