use std::{path::PathBuf, fs};
use eyre::Result;

pub fn app_dir() -> PathBuf {
    let home_directory = dirs::home_dir().expect("Failed to find home directory");
    let app_dir = home_directory.join(".rauschen");
    app_dir
}

pub fn create_home_dir_if_not_exist() -> Result<()> {
    if app_dir().exists() {
        return Ok(())
    }
    fs::create_dir(&app_dir())?;
    Ok(())
}