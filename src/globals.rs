use std::{env, path::PathBuf, sync::LazyLock};

use crate::utils::filesystem::{ensure_dir, ensure_writable};

pub static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(init_data_path);
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_PATH.join("./config.json"));

fn init_data_path() -> PathBuf {
    let try_env = |var: &str, suffix: &str| {
        env::var(var)
            .map_err(|e| format!("Cannot read env var: {e}"))
            .and_then(|dir| {
                let path = PathBuf::from(dir).join(suffix);
                ensure_dir(&path)?;
                ensure_writable(&path)?;

                Ok(path)
            })
    };

    let data_dir = try_env("XDG_DATA_HOME", "elysia")
        .or_else(|_| try_env("HOME", ".local/share/elysia"))
        .or_else(|_| try_env("PWD", "elysia"));

    // TODO: show this error in a popup
    data_dir.unwrap_or_else(|_| {
        panic!(
            "Cannot find a writable directory for data files (tried: $XDG_DATA_HOME/elysia, $HOME/.local/share/elysia, $PWD/elysia)"
        )
    })
}
