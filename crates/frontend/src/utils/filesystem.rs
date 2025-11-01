use std::{fs, path::Path};

pub fn ensure_dir(dir: &Path) -> Result<(), String> {
    let exists =
        fs::exists(dir).map_err(|e| format!("Cannot check if directory {dir:?} exists: {}", e))?;
    if exists {
        return Ok(());
    }

    fs::create_dir_all(dir).map_err(|e| format!("Cannot create directory {dir:?} : {e}"))?;

    Ok(())
}

pub fn ensure_writable(path: &Path) -> Result<(), String> {
    let metadata =
        fs::metadata(path).map_err(|e| format!("Cannot check if path {path:?} exists: {}", e))?;
    if metadata.permissions().readonly() {
        return Err(format!("Path is read-only: {path:?}"));
    }

    Ok(())
}

pub fn ensure_or_default<'a>(path: &'a Path, default: &Path) -> Result<&'a Path, String> {
    match ensure_dir(path) {
        Ok(()) => Ok(path),
        Err(e) => {
            println!("Error when creating/loading directory from config, will use default: {e}");
            ensure_dir(default)?;
            Ok(path)
        }
    }
}
