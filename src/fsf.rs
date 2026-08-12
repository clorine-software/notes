use anyhow::Result;
use std::env;
use std::path::PathBuf;
use tokio::fs;

use crate::consts::*;
use crate::logic;

pub async fn get_data() -> Result<logic::Root> {
    let data_dir = get_data_dir().await?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).await?;
    }
    let data_file = data_dir.join(DATA_FILE_NAME);
    if data_file.exists() {
        Ok(ron::from_str(&fs::read_to_string(data_file).await?)?)
    } else {
        Ok(logic::Root {
            special: Vec::new(),
            common: Vec::new(),
        })
    }
}

pub async fn save_data(data: logic::Root) -> Result<()> {
    let data_dir = get_data_dir().await?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).await?;
    }
    let data_file = data_dir.join(DATA_FILE_NAME);
    fs::write(
        data_file,
        ron::ser::to_string_pretty(
            &data,
            ron::ser::PrettyConfig::new()
                .depth_limit(4)
                .indentor("  ".to_string()),
        )?,
    )
    .await?;
    Ok(())
}

pub async fn get_config() -> Result<logic::Config> {
    let conf_dir = get_config_dir().await?;
    if !conf_dir.exists() {
        fs::create_dir_all(&conf_dir).await?;
    }
    let conf_file = conf_dir.join(CONFIG_FILE_NAME);
    if conf_file.exists() {
        Ok(toml::from_str(&fs::read_to_string(conf_file).await?)?)
    } else {
        Ok(logic::Config { style: None })
    }
}

async fn get_data_dir() -> Result<PathBuf> {
    Ok(get_data_home().await?.join(PROGRAM_NAME))
}

async fn get_config_dir() -> Result<PathBuf> {
    Ok(get_config_home().await?.join(PROGRAM_NAME))
}

#[cfg(target_os = "linux")]
async fn get_data_home() -> Result<PathBuf> {
    match env::var("XDG_DATA_HOME") {
        Ok(var) => Ok(PathBuf::from(var)),
        Err(e) => {
            eprintln!("$XDG_DATA_HOME error: {e}, fallback to .local/share");
            let home = env::var("HOME")?;
            Ok(PathBuf::from(home).join(".local/share"))
        }
    }
}

#[cfg(target_os = "windows")]
async fn get_data_home() -> Result<PathBuf> {
    Ok(PathBuf::from(env::var("APPDATA")?))
}

#[cfg(target_os = "macos")]
async fn get_data_home() -> Result<PathBuf> {
    Ok(PathBuf::from(env::var("HOME")?).join("Library/Application Support"))
}

#[cfg(target_os = "linux")]
async fn get_config_home() -> Result<PathBuf> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => Ok(PathBuf::from(var)),
        Err(e) => {
            eprintln!("$XDG_CONFIG_HOME error: {e}, fallback to .config");
            let home = env::var("HOME")?;
            Ok(PathBuf::from(home).join(".config"))
        }
    }
}

#[cfg(target_os = "windows")]
async fn get_config_home() -> Result<PathBuf> {
    Ok(PathBuf::from(env::var("APPDATA")?))
}

#[cfg(target_os = "macos")]
async fn get_config_home() -> Result<PathBuf> {
    Ok(PathBuf::from(env::var("HOME")?).join("Library/Preferences"))
}
