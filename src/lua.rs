use anyhow::Result;
use serde::Deserialize;
use home::home_dir;
use tokio::fs;
use mlua::Lua;

// config.toml struct
#[derive(Debug, Deserialize)]
pub struct Config {
    pub style: Option<Style>,
}

// config.toml [styles] struct
#[derive(Debug, Deserialize)]
pub struct Style {
    #[serde(default)]
    pub before_text: String,

    #[serde(default)]
    pub after_text: String,

    #[serde(default)]
    pub before_specials: String,

    #[serde(default)]
    pub after_specials: String,

    #[serde(default)]
    pub before_commons: String,

    #[serde(default)]
    pub after_commons: String,

    #[serde(default)]
    pub before_special_unit: String,

    #[serde(default)]
    pub after_special_unit: String,

    #[serde(default)]
    pub before_common_unit: String,

    #[serde(default)]
    pub after_common_unit: String,
}

pub async fn get_config() -> Result<Config> {
    let lua = Lua::new();
    
    let mut data_dir = home_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    data_dir.push(".clorine/config/notes");
    if !data_dir.exists() { fs::create_dir_all(&data_dir).await?; }
    let data_file = data_dir.join("config.lua");
    if data_file.exists() {
        Ok(toml::from_str(&fs::read_to_string(data_file).await?)?)
    } else {
        Ok(Config{style: None})
    }
}
