use freya::prelude::{Readable, Signal};
use serde::{Deserialize, Serialize};

use crate::settings::{GlobalSettings, InstalledGame};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub version: String,
}

impl Wine {
    pub fn run_game(&self, game: &InstalledGame) {
        let ctx = &dioxus::hooks::use_context::<Signal<GlobalSettings>>();
        let settings = &ctx.read();
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
        )
    }
}
